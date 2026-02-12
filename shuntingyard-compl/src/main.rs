use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Variable(String),
    Operator(char),
    Function(String, usize), // Function name + argument count
    Comma,
    LeftParen,
    RightParen,
}

// Determines operator precedence
fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1, // Lowest precedence
        '*' | '/' => 2, // Higher precedence
        _ => 0,         // Default case
    }
}

// Convert infix notation to postfix (Reverse Polish Notation - RPN)
fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new(); // Output queue (postfix expression)
    let mut operators = Vec::new(); // Operator stack
    let mut arg_count_stack = Vec::new(); // Stack for function argument count

    for token in tokens {
        match token {
            // Numbers and variables go directly to output
            Token::Number(_) | Token::Variable(_) => output.push(token),

            // Functions go on the operator stack
            Token::Function(name, _) => {
                operators.push(Token::Function(name, 0)); 
                arg_count_stack.push(1); // Initialize argument count
            }

            // Comma separates function arguments
            Token::Comma => {
                while let Some(top) = operators.last() {
                    // Pop until we find a left parenthesis
                    if !matches!(top, Token::LeftParen) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                // Increase argument count
                if let Some(last) = arg_count_stack.last_mut() {
                    *last += 1;
                }
            }

            // Operators are pushed to stack, but we check precedence
            Token::Operator(op) => {
                while let Some(&Token::Operator(top_op)) = operators.last() {
                    if precedence(top_op) >= precedence(op) {
                        output.push(operators.pop().unwrap()); // Pop higher precedence ops
                    } else {
                        break;
                    }
                }
                operators.push(Token::Operator(op)); // Push current operator
            }

            // Left parenthesis goes directly on the stack
            Token::LeftParen => operators.push(Token::LeftParen),

            // Right parenthesis triggers popping until left parenthesis is found
            Token::RightParen => {
                while let Some(top) = operators.last() {
                    if !matches!(top, Token::LeftParen) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.pop(); // Remove '(' from stack

                // If a function is on top, pop it to output
                if let Some(Token::Function(name, _)) = operators.last().cloned() {
                    let arg_count = arg_count_stack.pop().unwrap_or(1);
                    output.push(Token::Function(name, arg_count));
                    operators.pop();
                }
            }
        }
    }

    // Pop any remaining operators
    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

// Build an Abstract Syntax Tree (AST) from postfix notation
fn build_ast(postfix_tokens: Vec<Token>) -> DiGraph<String, ()> {
    let mut graph = DiGraph::new(); // Directed graph
    let mut stack = Vec::new(); // Stack to hold nodes

    for token in postfix_tokens {
        match token {
            Token::Number(n) => stack.push(graph.add_node(n.to_string())), // Create number node
            Token::Variable(var) => stack.push(graph.add_node(var)), // Create variable node
            
            // If token is an operator, pop two operands and create an operator node
            Token::Operator(op) => {
                let right = stack.pop().expect("Right operand missing");
                let left = stack.pop().expect("Left operand missing");

                let op_node = graph.add_node(op.to_string()); // Create operator node
                graph.add_edge(op_node, left, ()); // Connect left operand
                graph.add_edge(op_node, right, ()); // Connect right operand

                stack.push(op_node); // Push the new subtree root back onto stack
            }

            // If token is a function, pop arguments and create function node
            Token::Function(name, arg_count) => {
                let func_node = graph.add_node(format!("{}()", name)); // Function node
                let mut args = Vec::new();

                // Pop the correct number of arguments
                for _ in 0..arg_count {
                    args.push(stack.pop().expect("Missing function arguments"));
                }

                // Attach arguments to function node
                for arg in args.into_iter().rev() {
                    graph.add_edge(func_node, arg, ());
                }

                stack.push(func_node); // Push function node onto stack
            }
            _ => {}
        }
    }

    graph
}

fn main() {
    let tokens = vec![
        Token::LeftParen,
        Token::Number(2),
        Token::Operator('+'),
        Token::Variable("x".to_string()),
        Token::RightParen,
        Token::Operator('*'),
        Token::Number(4),
        Token::Operator('+'),
        Token::Function("sum".to_string(), 2),
        Token::LeftParen,
        Token::Number(3),
        Token::Comma,
        Token::Number(5),
        Token::RightParen,
    ];

    let postfix_tokens = shunting_yard(tokens); // Convert to postfix
    let ast = build_ast(postfix_tokens); // Construct AST

    // Print AST in Graphviz DOT format
    println!("{:?}", Dot::with_config(&ast, &[Config::EdgeNoLabel]));
}