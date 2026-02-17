mod constants;
mod visitor;
mod template_engine;

use template_engine::*;
use crate::prelude::*;
use constants::*;
use visitor::*;

pub fn run_code_generation(ast: &syntax::Tree) -> anyhow::Result<()> {
    let mut helper = Helper::default();
    let mut symbol_table = SymbolTable::default();

    let pbar = Arc::new(ProgressBar::new(ast.data.node_count() as u64));
    pbar.set_style(ProgressStyle::default_bar()
        .template("Generating code {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})\n{msg:.magenta}")
        .unwrap()
        .progress_chars("#>-"));

    let pbar_clone = Arc::clone(&pbar);

    // Find the tree root by looking for the node with no incoming edges
    let root = ast.data.node_indices().find(|&node| ast.data.neighbors_directed(node, petgraph::Direction::Incoming).count() == 0).unwrap();

    match preorder_traversal(&mut helper, &mut symbol_table, root, &ast, 0, pbar_clone) {
        Err(e) => {
            eprintln!("{}", "Semantic error found\n".red());

            match e {
                Error::User(..) => eprintln!("{}\n", e.to_string().red()),
                Error::Compiler(_) => eprintln!("{}\n", e.to_string().red()),
            }

            eprintln!();
            bail!("A semantic error occurred")
        }
        Ok(()) => {}
    }

    pbar.finish_with_message("Code generation complete OwO!\n");
    println!("{:?} test",symbol_table);
    create_files(&mut symbol_table); 
    Ok(())
}