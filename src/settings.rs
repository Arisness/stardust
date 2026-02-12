use crate::prelude::*;
use anyhow::Result;
use clap::{ Arg, Command, ArgMatches };

pub struct Settings {
    pub file_path: PathBuf,
    pub display_ast: bool,
}

impl Settings {
    pub fn new(matches: &ArgMatches) -> Result<Self> {
        let file_path_str = matches
            .get_one::<String>("path")
            .cloned()
            .ok_or(anyhow!("Unexpected value for `path` argument"))?;

        let file_path = PathBuf::try_from(file_path_str)?;

        if !file_path.is_file() {
            bail!("The provided path is not a valid file");
        }

        let display_ast = matches.get_flag("tree");

        Ok(Self {
            file_path,
            display_ast,
        })
    }
}

pub fn get_command() -> Command {
    Command::new("Pascal Compiler Prototype")
        .version("0.1.0")
        .author("Kaucrow")
        .about(r#"Compiler for Pascal source code, except it doesn't... compile stuff?
Idk it kinda just runs a lexical, syntactic, and semantic
analysis but doesn't really compile anything lolz

...it's doing its best, okay?"#
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .num_args(1)
                .required(true)
                .help("Pascal source code file path")
        )
        .arg(
            Arg::new("tree")
                .short('t')
                .long("tree")
                .num_args(0)
                .required(false)
                .help("Displays a graphical representation of the generated AST (Abstract Syntax Tree)")
        )
}

pub fn get_matches() -> ArgMatches {
    get_command().get_matches()
}