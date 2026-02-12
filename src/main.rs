use stardust::prelude::*;
use stardust::settings::{ self, Settings };
use anyhow::Result;

fn main() -> Result<()> {
    let matches = settings::get_matches();

    // If the "help" option was passed, print the help screen and exit
    if matches.contains_id("help") {
        let mut cmd = settings::get_command();
        cmd.print_help().expect("Failed to print help");
        println!();
        return Ok(());
    }

    // Build the settings from the command line arguments
    let settings = Settings::new(&matches)?;

    // Run the lexical analysis and print the resulting tokens
    let tokens_list = lexicon::run_lexical_analysis(&settings)?;
    println!("\n\nLexical tokens:");
    for line in &tokens_list {
        println!("{:?}", line);
    }
    println!();

    // Run the syntactic analysis and display the resulting AST
    let ast = syntax::run_syntactic_analysis(tokens_list, &settings)?;

    // Run the semantic analysis
    semantic::run_semantic_analysis(ast)?;

    Ok(())
}