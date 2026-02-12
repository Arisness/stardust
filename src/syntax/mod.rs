pub mod automata;
pub mod tree;
mod states;

pub use automata::*;
pub use tree::{ Tree, TreeAction, Node, Id };

use crate::prelude::*;
use crate::settings::Settings;
use anyhow::Result;
use std::process::Command;

pub fn run_syntactic_analysis(tokens_list: Vec<Vec<Token>>, settings: &Settings) -> Result<Tree> {
    // Initialize the Syntax PDA and AST
    let mut pda = PDA::new();
    let mut ast = Tree::new();

    let tokens_amt: usize = tokens_list.iter().map(|v| v.len()).sum();

    let syntax_bar = ProgressBar::new(tokens_amt as u64);
    syntax_bar.set_style(ProgressStyle::default_bar()
        .template("Analyzing syntax {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})\n{msg:.magenta}")
        .unwrap()
        .progress_chars("#>-"));

    for (i, line) in tokens_list.iter().enumerate() {
        let mut line = line.iter().peekable();

        while let Some(token) = line.next() {
            let next_token = line.peek();

            pda.transition(&token, next_token, &mut ast, i + 1).or_else(|err| {
                eprintln!("{} {}\n", "Syntax error found in line".red(), i + 1);
                eprintln!("{}", "Automata status:".magenta());
                eprintln!("{} {:?}", "- Mode: ".yellow(), pda.mode);
                eprintln!("{} {}", "- State: ".yellow(), pda.state);
                eprintln!("{} {:?}", "- Input: ".yellow(), token);
                eprintln!("{} {:?}\n", "- Stack: ".yellow(), pda.stack);
                eprintln!();
                bail!(err);
            })?;

            syntax_bar.inc(1);
        }
    }

    syntax_bar.finish_with_message("Syntactical analysis complete! :D\n");

    if settings.display_ast {
        println!("A browser window with the Abstract Syntactical Tree diagram should open now.\nIf it doesn't, copy and paste the following link in a browser to view it:");

        let ast_dot = format!("{:?}", Dot::with_config(&ast.data, &[Config::EdgeNoLabel]));
        let encoded = encode(&ast_dot);
        let url = format!("https://quickchart.io/graphviz?graph={}\n", encoded);

        println!("{}", url.green().bold());

        open_in_browser(&url);
    }

    Ok(ast)
}

fn open_in_browser(url: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", url]) // Opens in default browser
            .spawn()
            .expect("Failed to open URL");
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .spawn()
            .expect("Failed to open URL");
    } else {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
            .expect("Failed to open URL");
    }
}