use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Variable(String),
    Operator(char),
}

// Helper function to determine operator precedence
fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

// Converts infix to postfix using Shunting Yard Algorithm
fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) | Token::Variable(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(&Token::Operator(top_op)) = operators.last() {
                    if precedence(top_op) >= precedence(op) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(Token::Operator(op));
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

// Builds an AST from a postfix expression using petgraph
fn build_ast(postfix_tokens: Vec<Token>) -> DiGraph<String, ()> {
    let mut graph = DiGraph::new();
    let mut stack = Vec::new();

    for token in postfix_tokens {
        match token {
            Token::Number(n) => {
                let node = graph.add_node(n.to_string());
                stack.push(node);
            }
            Token::Variable(var) => {
                let node = graph.add_node(var);
                stack.push(node);
            }
            Token::Operator(op) => {
                let right = stack.pop().expect("Right operand missing");
                let left = stack.pop().expect("Left operand missing");

                let op_node = graph.add_node(op.to_string());
                graph.add_edge(op_node, left, ());
                graph.add_edge(op_node, right, ());

                stack.push(op_node);
            }
        }
    }

    graph
}

fn main() {
    let tokens = vec![
        Token::Number(2),
        Token::Operator('+'),
        Token::Variable("x".to_string()),
        Token::Operator('*'),
        Token::Number(4),
    ];

    let postfix_tokens = shunting_yard(tokens);
    let ast = build_ast(postfix_tokens);

    // Print the AST in Graphviz DOT format
    println!("{:?}", Dot::with_config(&ast, &[Config::EdgeNoLabel]));
}