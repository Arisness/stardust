use crate::prelude::*;
use super::constants::Error;
use syntax::tree::{Node, Id};

#[derive(Debug, Default)]
pub struct Helper {
    act_stack: Vec<(Id, i32)>,
    path: Option<String>,
    class: Option<String>,
    method: Option<String>,
}

#[derive(Debug, Default)]
pub struct MethodRecord {
    pub params: Vec<String>,
}

#[derive(Debug, Default)]
pub struct ClassRecord {
    pub methods: HashMap<String, MethodRecord>, 
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub records: HashMap<String, HashMap<String, ClassRecord>>,
}

pub fn preorder_traversal<'a> (
    //dict: &mut Dict,
    helper: &mut Helper,
    symbol_table: &mut SymbolTable,
    node_idx: NodeIndex,
    ast: &syntax::Tree,
    depth: i32,
    pbar: Arc<ProgressBar>,
) -> Result<(), Error> {
    let act_stack = &mut helper.act_stack;

    let (node, line) = &ast.data[node_idx];

    // Clear up stack actions that have finished
    while let Some((_, act_depth)) = act_stack.last() {
        if depth <= *act_depth {    // If we're moving up the tree
            act_stack.pop();    // Remove the last action
        } else {
            break;  // Stop when we reach a higher node
        }
    }

    match node {
        Node::Id(Id::Entry) => {}

        Node::Id(Id::Path) => {
            act_stack.push((Id::Path, depth));
        }

        Node::Id(Id::Method) => {
            act_stack.push((Id::Method, depth));
        }

        Node::Id(Id::Params) => {
            act_stack.push((Id::Params, depth));
        }

        Node::Val(Token::String(string)) => {
            match act_stack.last() {
                // Path string
                Some((Id::Path, _)) => {
                    helper.path = Some(string.clone());

                    symbol_table.records
                        .entry(string.clone())
                        .or_insert_with(HashMap::new);
                }
                _ => unimplemented!()
            }
        }

        Node::Val(Token::Identifier(identifier)) => {
            match act_stack.last() {
                // Class identifier
                Some((Id::Path, _)) => {
                    helper.class = Some(identifier.clone());

                    let path = helper.path.as_ref()
                        .ok_or(Error::Compiler("No current path".to_string()))?;

                    let class_map = symbol_table.records.get_mut(path)
                        .ok_or(Error::Compiler("Path doesn't exist".to_string()))?;

                    class_map.insert(identifier.clone(), ClassRecord::default());
                },

                Some((Id::Method, _)) => {
                    helper.method = Some(identifier.clone());

                    let path = helper.path.as_ref()
                        .ok_or(Error::Compiler("No current path".to_string()))?;

                    let class = helper.class.as_ref()
                        .ok_or(Error::Compiler("No current class".to_string()))?;

                    let methods_map = &mut symbol_table.records.get_mut(path)
                        .ok_or(Error::Compiler("Path doesn't exist".to_string()))?
                        .get_mut(class)
                        .ok_or(Error::Compiler(format!("Class {class} doesn't exist")))?
                        .methods;

                    methods_map.insert(identifier.clone(), MethodRecord::default());
                },

                Some((Id::Params, _)) => {
                    let path = helper.path.as_ref()
                        .ok_or(Error::Compiler("No current path".to_string()))?;

                    let class = helper.class.as_ref()
                        .ok_or(Error::Compiler("No current class".to_string()))?;

                    let method = helper.method.as_ref()
                        .ok_or(Error::Compiler("No current method".to_string()))?;

                    let params = &mut symbol_table.records.get_mut(path)
                        .ok_or(Error::Compiler("Path doesn't exist".to_string()))?
                        .get_mut(class)
                        .ok_or(Error::Compiler("Class doesn't exist".to_string()))?
                        .methods
                        .get_mut(method)
                        .ok_or(Error::Compiler("Method doesn't exist".to_string()))?;

                    params.params.push(identifier.clone());
                }

                _ => unimplemented!(),
            }
        }

        _ => unimplemented!("{:?}", node)
    }

    pbar.inc(1);
    let children: Vec<_> = ast.data.neighbors(node_idx).collect();
    for &child in children.iter().rev() {
        match node {
            _ => {
                preorder_traversal(helper, symbol_table, child, ast, depth + 1, pbar.clone())?;
            }
        }
    }

    Ok(())
}