use crate::prelude::*;
use crate::settings::Settings;
use anyhow::Result;

fn count_file_lines(filename: &PathBuf) -> io::Result<u64> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count() as u64)
}

pub fn run_lexical_analysis(settings: &Settings) -> Result<Vec<Vec<Token>>> {
    let file_lines = count_file_lines(&settings.file_path)?;

    let lex_bar = ProgressBar::new(file_lines);
    lex_bar.set_style(ProgressStyle::default_bar()
        .template("Analyzing lexic {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})\n{msg:.magenta}")
        .unwrap()
        .progress_chars("#>-"));

    let file = File::open(settings.file_path.clone())?;
    let reader = io::BufReader::new(file);

    // Build Aho-Corasick state machines
    let ahocors: Vec<(TokenProto, AhoCorasick)> = LEX_AHOCOR_DICT.clone().into_iter().map(|(token_proto, matches)| {
        (
            token_proto,
            AhoCorasick::new(matches).unwrap()
        )
    }).collect();

    let mut tokens_list: Vec<Vec<Token>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut line = line.as_str().trim_start();
        let mut line_tokens: Vec<Token> = Vec::new();

        while !line.is_empty() {
            let mut found_match = false;

            // Try to find a match using Aho-Corasick algorithm
            for (token_proto, ahocor) in &ahocors {
                if let Some(mat) = ahocor.find(&line) {
                    // Ensure the token matched starts at the beginning of the line
                    if mat.start() != 0 {
                        continue;
                    }

                    found_match = true;

                    let mat_str = &line[mat.start()..mat.end()];

                    line_tokens.push(token_proto.to_token(mat_str.to_string()));
                    line = &line[mat.end()..].trim_start();

                    break;
                }
            }

            // If no match was found, try to find a match with Regex
            if !found_match {
                for (token_proto, regex) in LEX_REGEX_DICT.iter() {
                    if let Some(mat) = regex.find(&line) {
                        // Ensure the token matched starts at the beginning of the line
                        if mat.start() != 0 {
                            continue;
                        }

                        found_match = true;

                        let mat_start = mat.start();
                        let mat_end = mat.end();

                        let token_proto = token_proto.clone();

                        let mat_str = &line[mat_start..mat_end];

                        if token_proto != TokenProto::Comment {
                            line_tokens.push(token_proto.to_token(mat_str.to_string()));
                        }
                        line = &line[mat_end..].trim_start();

                        break;
                    }
                }
            }

            if !found_match {
                panic!("{}", format!(
                    "Failed to find a suitable token for the remaining line: `{}`",
                    line
                ));
            }
        }

        tokens_list.push(line_tokens);

        lex_bar.inc(1);
    }

    lex_bar.finish_with_message("Lexical analysis complete! :)");

    Ok(tokens_list)
}