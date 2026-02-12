pub mod constants;

use crate::prelude::*;
use constants::*;
use syntax::tree::{ Node, Id };

pub struct Dict {
    var: VarDict,
    func: FuncDict,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            var: VarDict::new(),
            func: FuncDict::new(),
        }
    }
}

pub struct Helper {
    scope_stack: Vec<(Scope, i32)>,
    act_stack: Vec<(Id, i32)>,
    temp_var: String,
}

impl Helper {
    pub fn new() -> Self {
        Self {
            scope_stack: vec![(Scope::Global, -1)],
            act_stack: Vec::new(),
            temp_var: String::new(),
        }
    }
}

pub fn preorder_traversal<'a> (
    dict: &mut Dict,
    helper: &mut Helper,
    node_idx: NodeIndex,
    ast: &syntax::Tree,
    depth: i32,
    pbar: Arc<ProgressBar>,
) -> Result<(), Error> {
    let act_stack = &mut helper.act_stack;
    let temp_var = &mut helper.temp_var;

    let (node, line) = &ast.data[node_idx];

    // Clear up stack actions that have finished
    while let Some((_, act_depth)) = act_stack.last() {
        if depth <= *act_depth {    // If we're moving up the tree
            act_stack.pop(); // Remove the last action
        } else {
            break; // Stop when we reach a higher node
        }
    }

    // Clear up scopes that have finished
    while let Some((_, scope_depth)) = helper.scope_stack.last() {
        if depth <= *scope_depth {    // If we're moving up the tree
            helper.scope_stack.pop(); // Remove the last action
        } else {
            break; // Stop when we reach a higher node
        }
    }

    match node {
        Node::Id(id) if *id == Id::Var || *id == Id::Assign || *id == Id::FuncDecl => {
            act_stack.push((*id, depth));
        }
        Node::Id(Id::Return) => {
            act_stack.push((Id::Return, depth));
        }
        Node::Val(val) => {
            match val {
                Token::Identifier(name) => {
                    if let Some(act) = act_stack.last() {
                        match act {
                            // Variable declaration
                            (Id::Var, _) | (Id::FuncDecl, _) => {
                                *temp_var = name.to_string();
                            }
                            _ => {}
                        }
                    }
                }
                Token::MethodIdent(name) => {
                    if let Some(act) = act_stack.last() {
                        match act {
                            // Function declaration
                            (Id::FuncDecl, _) => {
                                // Remove 2 to the depth to compensate for the fact that a function declaration is higher
                                // than its identifier in the AST
                                helper.scope_stack.push((Scope::Func(name.to_string()), depth - 2));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        } 
        _ => {}
    }

    pbar.inc(1);

    let children: Vec<_> = ast.data.neighbors(node_idx).collect();
    for &child in children.iter().rev() {
        match node {
            Node::Id(Id::Return) => {
                let (scope, _) = helper.scope_stack.last().ok_or(Error::Compiler("No scope".to_string()))?;

                let exp_ret_type = {
                    if let Scope::Func(name) = scope {
                        Ok(dict.func.get(scope).ok_or(Error::User(
                            *line,
                            format!("Function '{}' has not been defined", name)
                        ))?
                        .returntype
                        .as_ref()
                        .ok_or(Error::Compiler("Function has no return type".to_string()))?)
                    } else {
                        Err(Error::Compiler("The top of the scope stack doesn't contain a function".to_string()))
                    }
                }?;

                let ret_type = get_expression_type(child, VecDeque::new(), helper, dict, ast)?;

                if *exp_ret_type != ret_type {
                    if let Scope::Func(name) = scope {
                        return Err(Error::User(
                            *line,
                            format!("Function {} expected {:?} as return type, but got {:?} instead", name, exp_ret_type, ret_type)
                        ));
                    } else {
                        return Err(Error::Compiler(format!(
                            "Unexpected scope found: {:?}", scope
                        )));
                    }
                }
            }
            Node::Id(Id::Assign) => {
                let var_node_idx = children.get(1).ok_or(Error::Compiler("Assign node has no left hand side child node".to_string()))?;
                let (var_node, _) = &ast.data[var_node_idx.clone()];

                if let Node::Val(Token::Identifier(var_name)) = var_node {
                    let (scope, _) = helper.scope_stack.last().ok_or(Error::Compiler("No scope".to_string()))?;

                    let var = dict.var.get(scope).ok_or(Error::Compiler("Scope not found".to_string()))?
                        .get(var_name.as_str())
                        .ok_or(Error::User(
                            *line,
                            format!("Variable '{}' does not exist in scope", var_name)
                        ))?;

                    let exp_type = &var.datatype;

                    let expression_root = children.get(0).ok_or(Error::Compiler("Assign node has no right hand side child node".to_string()))?;
                    let datatype = get_expression_type(*expression_root, VecDeque::new(), helper, dict, ast)?;

                    if *exp_type != datatype {
                        return Err(Error::User(
                            *line,
                            format!("Invalid assignment: Cannot assign type {:?} to a variable of type {:?}", datatype, exp_type)
                        ));
                    }

                    // Remove the "variable uninitialized" warning
                    dict.var
                        .get_mut(scope)
                        .ok_or(Error::Compiler("Scope not found".to_string()))?
                        .get_mut(var_name)
                        .ok_or(Error::Compiler("Variable does not exist in scope".to_string()))?
                        .uninit = false;
                } else {
                    return Err(Error::User(
                        *line,
                        format!("Attempted to assign a value to something other than a variable")
                    ));
                }
            }
            Node::Id(Id::If) => {
                let cond_node_idx = children.last().ok_or(Error::Compiler("'If' node has no children".to_string()))?;

                let datatype = get_expression_type(*cond_node_idx, VecDeque::new(), helper, dict, ast)?;

                if datatype != DataType::Bool {
                    return Err(Error::User(
                        *line,
                        format!("The if expression's condition expected type Bool but got type {:?} instead", datatype)
                    ));
                }

                for child in children.iter().take(children.len() - 1) {
                    preorder_traversal(dict, helper, *child, ast, depth, pbar.clone())?;
                }
            }
            _ => {
                preorder_traversal(dict, helper, child, ast, depth + 1, pbar.clone())?;
            }
        }
    }

    Ok(())
}

pub fn get_expression_type(node: NodeIndex, mut results: VecDeque<DataType>, helper: &Helper, dict: &Dict, ast: &syntax::Tree) -> Result<DataType, Error> {
    let children: Vec<_> = ast.data.neighbors(node).collect();

    for child in children.iter().rev() {
        let result = get_expression_type(*child, VecDeque::new(), helper, dict, ast)?;
        results.push_back(result);
    }

    let (node, line) = &ast.data[node];

    match node {
        Node::Val(Token::Number(_)) => Ok(DataType::Int),
        Node::Val(Token::Boolean(_)) => Ok(DataType::Bool),
        Node::Val(Token::String(_)) => Ok(DataType::String),
        Node::Val(Token::Char(_)) => Ok(DataType::Char),
        Node::Val(Token::Identifier(name)) => {
            let mut datatype: Option<DataType> = None;

            for (scope, _) in helper.scope_stack.iter().rev() {
                let variable = &dict.var.get(scope).ok_or(Error::Compiler("Scope not found".to_string()))?
                    .get(name.as_str());

                if let Some(variable) = variable {
                    datatype = Some(variable.datatype.clone());
                    break;
                }
            }

            Ok(datatype.ok_or(Error::Compiler("Variable does not exist in scope".to_string()))?)
        }
        _ => Err(Error::Compiler(format!("Unexpected token: {:?}", node)))
    }
}

pub fn run_semantic_analysis(ast: syntax::Tree) -> anyhow::Result<()> {
    let mut dict = Dict::new();
    let mut helper = Helper::new();

    let pbar = Arc::new(ProgressBar::new(ast.data.node_count() as u64));
    pbar.set_style(ProgressStyle::default_bar()
        .template("Analyzing semantics {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})\n{msg:.magenta}")
        .unwrap()
        .progress_chars("#>-"));

    let pbar_clone = Arc::clone(&pbar);

    // Find the tree root by looking for the node with no incoming edges
    let root = ast.data.node_indices().find(|&node| ast.data.neighbors_directed(node, petgraph::Direction::Incoming).count() == 0).unwrap();

    match preorder_traversal(&mut dict, &mut helper, root, &ast, 0, pbar_clone) {
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

    pbar.finish_with_message("Semantic analysis complete OwO!\n");

    // If there are any warnings, display them
    for (scope, variables) in dict.var.iter() {
        for (var_name, var_data) in variables {
            if !var_data.uninit {
                continue;
            }

            let scope_message = match scope {
                Scope::Global => &format!("global variable '{}'", var_name),
                Scope::Main => &format!("variable '{}' in main", var_name),
                Scope::Func(func_name) => &format!("variable '{}' in function '{}'", var_name, func_name),
            };

            eprintln!(
                "{} {}{} {} has not been initialized",
                "Warning: Line".yellow(),
                var_data.decl_line.to_string().yellow(),
                ":".yellow(),
                scope_message
            );

            eprintln!();
        }
    }

    Ok(())
}