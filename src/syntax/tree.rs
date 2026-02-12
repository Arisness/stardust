use crate::prelude::*;
use anyhow::Result;
use petgraph::graph::{DiGraph, NodeIndex}; // explicit imports help clarity
use petgraph::Direction;

#[derive(Debug, Clone)]
pub enum Node {
    Id(Id),
    Val(Token),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Id {
    Entry,

    At,
    Path,
    Class,
    Method,

    Program,
    Name,
    Declarations,
    Var,
    FuncDecl,
    Params,
    ReturnType,
    Body,
    Return,
    Main,
    Assign,
    Writeln,
    If,
    Else,
}

pub enum TreeAction {
    AddNode(Option<Node>),
    AppendChild(Option<Node>),
    GoUp,
}

pub struct Tree {
    pub curr_node: Option<NodeIndex>,
    // Storing (Node, usize) to keep track of line numbers
    pub data: DiGraph<(Node, usize), ()>, 
}

impl Tree {
    pub fn new() -> Self {
        let mut data = DiGraph::new();

        // Create the Entry node immediately (using line 0 as it is synthetic)
        let root_node = data.add_node((Node::Id(Id::Entry), 0));

        Self {
            curr_node: Some(root_node),
            data,
        }
    }

    pub fn add_node(&mut self, value: Node, line: usize) {
        if let Some(parent) = self.curr_node {
            let new_node = self.data.add_node((value, line));
            self.data.add_edge(parent, new_node, ());
            // Move into the new node (descend)
            self.curr_node = Some(new_node);
        } else {
            self.curr_node = Some(self.data.add_node((value, line)));
        }
    }

    pub fn append_child(&mut self, value: Node, line: usize) -> Result<()> {
        if let Some(parent) = self.curr_node {
            let child = self.data.add_node((value, line));
            self.data.add_edge(parent, child, ());
            // With "append_child" we stay at the parent
            Ok(())
        } else {
            bail!("Failed to append child to AST node: No current node");
        }
    }

    pub fn go_up(&mut self) -> Result<()> {
        if let Some(curr_node_idx) = self.curr_node {
            // Find the parent of the current node
            if let Some(parent) = self.data.neighbors_directed(curr_node_idx, Direction::Incoming).next() {
                self.curr_node = Some(parent);
            } else {
                bail!("Already at the root (Entry) node");
            }
        }
        Ok(())
    }

    pub fn get_preorder_nodes(&self) -> Vec<(NodeIndex, i32)> {
        let tree = &self.data;

        // Find the tree root by looking for the node with no incoming edges
        let root = tree.node_indices().find(|&node| tree.neighbors_directed(node, petgraph::Direction::Incoming).count() == 0).unwrap();
        let mut result = Vec::new();
        let mut stack = vec![(root, 0)];

        while let Some((node, depth)) = stack.pop() {
            result.push((node, depth));

            // Collect children in left-to-right order
            let children: Vec<_> = tree.neighbors(node).collect();

            // Push children in reverse order so the leftmost child is processed first
            for &child in children.iter() {
                stack.push((child, depth + 1));
            }
        }
        result
    }
}