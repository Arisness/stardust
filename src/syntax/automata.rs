use std::hash::Hash;

use crate::prelude::*;
use super::{ TreeAction, Tree, Node };
use anyhow::Result;

#[derive(Debug)]
pub enum StackElem {
    // Normal
    Path,
    Class,
    Method,
    MethodParams,

    FuncReturn,
    Returned,
    VarBegun,
    VarCanExit,
    Declarations,
    Main,
    If,
    Else,
    // Expression parsing
    LParen,
    Func,
    // Builtin functions
    Writeln,
}

#[derive(PartialEq)]
pub enum StackType {
    // Normal
    Path,
    Class,
    Method,
    MethodParams,

    FuncReturn,
    Returned,
    VarBegun,
    VarCanExit,
    Declarations,
    Main,
    If,
    Else,
    // Expression parsing
    LParen,
    Func,
    // Builtin functions
    Writeln,
}

impl StackType {
    fn to_stack_elem(&self, token: Token) -> Result<StackElem> {
        match self {
            Self::Path => Ok(StackElem::Path),
            Self::Class => Ok(StackElem::Class),
            Self::Method => Ok(StackElem::Method),
            Self::MethodParams => Ok(StackElem::MethodParams),

            Self::FuncReturn => Ok(StackElem::FuncReturn),
            Self::Returned => Ok(StackElem::Returned),
            Self::VarBegun => Ok(StackElem::VarBegun),
            Self::VarCanExit => Ok(StackElem::VarCanExit),
            Self::Declarations => Ok(StackElem::Declarations),
            Self::Main => Ok(StackElem::Main),
            Self::If => Ok(StackElem::If),
            Self::Else => Ok(StackElem::Else),
            Self::LParen => Ok(StackElem::LParen),
            Self::Func => Ok(StackElem::Func),
            Self::Writeln => Ok(StackElem::Writeln),
        }
    }
}

impl StackElem {
    fn to_stacktype(&self) -> StackType {
        match self {
            Self::Path => StackType::Path,
            Self::Class => StackType::Class,
            Self::Method => StackType::Method,
            Self::MethodParams => StackType::MethodParams,

            Self::FuncReturn => StackType::FuncReturn,
            Self::Returned => StackType::Returned,
            Self::VarBegun => StackType::VarBegun,
            Self::VarCanExit => StackType::VarCanExit,
            Self::Declarations => StackType::Declarations,
            Self::Main => StackType::Main,
            Self::If => StackType::If,
            Self::Else => StackType::Else,
            Self::LParen => StackType::LParen,
            Self::Func => StackType::Func,
            Self::Writeln => StackType::Writeln,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mode {
    Normal,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ModeProto {
    Normal,
}

impl Mode {
    fn proto(&self) -> ModeProto {
        match self {
            Self::Normal => ModeProto::Normal,
        }
    }
}

pub enum Action {
    Tree(TreeAction),
    SwitchMode(Mode),
}

pub struct Transition {
    pub to_state: &'static str,
    pub action: Option<Vec<Action>>,
    pub input: Input,
    pub peek_input: Option<TokenProto>,
    pub cmp_stack: Option<StackType>,
    pub pop_stack: Option<StackType>,
    pub push_stack: Option<StackType>,
    pub peek_stack: Option<StackType>,
}

pub struct TransitionBuilder {
    to_state: &'static str,
    action: Option<Vec<Action>>,
    input: Input,
    peek_input: Option<TokenProto>,
    cmp_stack: Option<StackType>,
    pop_stack: Option<StackType>,
    push_stack: Option<StackType>,
    peek_stack: Option<StackType>,
}

impl TransitionBuilder {
    pub fn new(to_state: &'static str, input: Input) -> Self {
        Self {
            to_state,
            input,
            action: None,
            peek_input: None,
            cmp_stack: None,
            pop_stack: None,
            push_stack: None,
            peek_stack: None,
        }
    }

    pub fn action(mut self, action: Vec<Action>) -> Self {
        self.action = Some(action);
        self
    }

    pub fn peek_input(mut self, input: TokenProto) -> Self {
        self.peek_input = Some(input);
        self
    }

    pub fn push_stack(mut self, stack: StackType) -> Self {
        self.push_stack = Some(stack);
        self
    }

    pub fn pop_stack(mut self, stack: StackType) -> Self {
        self.pop_stack = Some(stack);
        self
    }

    pub fn cmp_stack(mut self, stack: StackType) -> Self {
        self.cmp_stack = Some(stack);
        self
    }

    pub fn peek_stack(mut self, stack: StackType) -> Self {
        self.peek_stack = Some(stack);
        self
    }

    pub fn build(self) -> Transition {
        Transition {
            to_state: self.to_state,
            action: self.action,
            input: self.input,
            peek_input: self.peek_input,
            cmp_stack: self.cmp_stack,
            pop_stack: self.pop_stack,
            push_stack: self.push_stack,
            peek_stack: self.peek_stack,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExprHelper {
    pub output: Vec<Token>,
    pub operators: Vec<Token>,
    pub arg_count: Vec<usize>,
    pub arg_count_final: Vec<usize>,
}

impl ExprHelper {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            operators: Vec::new(),
            arg_count: Vec::new(),
            arg_count_final: Vec::new(),
        }
    }
}

pub enum Input {
    Any,
    Token(TokenProto),
}

impl Input {
    fn matches_token(&self, proto: &TokenProto) -> bool {
        match self {
            Input::Any => true,
            Input::Token(t) => t == proto,
        }
    }
}

pub struct PDA {
    pub state: &'static str,
    pub mode: Mode,
    states: HashMap<ModeProto, HashMap<&'static str, Vec<Transition>>>,
    pub stack: Vec<StackElem>,
}

impl PDA {
    pub fn new() -> Self {
        let mut pda = Self {
            state: "q_exp_at",
            mode: Mode::Normal,
            states: HashMap::from([
                (ModeProto::Normal, HashMap::new()),
            ]),
            stack: Vec::new(),
        };
        pda.build();
        pda
    }

    pub fn add_state(&mut self, name: &'static str, mode: ModeProto, transitions: Vec<Transition>) {
        let states = self.states.get_mut(&mode).expect(format!("Mode is not in PDA states: {:?}", mode).as_str());
        states.insert(name, transitions);
    }

    pub fn transition(&mut self, input: &Token, next_token: Option<&&Token>, tree: &mut Tree, line: usize) -> Result<()> {
        let states = self.states.get(&self.mode.proto()).expect(format!("Mode is not in PDA states: {:?}", self.mode).as_str());

        let transition = {
            let transitions = states.get(self.state).ok_or(
                anyhow!(format!("State `{}` not found in states table", self.state))
            )?;

            let next_stack = self.stack.last();
            let mut transition_ret: Option<&Transition> = None;

            for transition in transitions {
                // If the input matches
                if transition.input.matches_token(&input.proto()) {
                    // Check if a stack pop is required.
                    // stack_pop_matches is true if either the requirement is met or there's no requirement
                    let stack_pop_matches = transition.pop_stack.as_ref().map_or(true, |transition_pop| {
                        next_stack.as_ref().map_or(false, |stack_elem| stack_elem.to_stacktype() == *transition_pop)
                    });

                    // Check if a peek input match is required.
                    // peek_input_matches is true if either the requirement is met or there's no requirement
                    let peek_input_matches = transition.peek_input.as_ref().map_or(true, |peek_input|
                        next_token.as_ref().map_or(false, |t| *peek_input == t.proto())
                    );

                    // Check peek_stack condition (second last element of the stack)
                    // peek_stack_matches is true if either the requirement is met or there's no requirement
                    let peek_stack_matches = transition.peek_stack.as_ref().map_or(true, |peek_stack| {
                        self.stack.get(self.stack.len().saturating_sub(2)) // Safely get the second last element
                            .map_or(false, |stack_elem| stack_elem.to_stacktype() == *peek_stack) // Compare
                    });


                    if stack_pop_matches && peek_input_matches && peek_stack_matches {
                        if transition.cmp_stack.is_some() {
                            let mut cmp_succeed = false;
                            /*
                            if matches!(transition.cmp_stack, Some(StackType::ExpReturn)) {
                                if let Token::Identifier(val) = input {
                                    if let Some(StackElem::ExpReturn(exp_return)) = self.stack.last() {
                                        if exp_return == val {
                                            cmp_succeed = true;
                                        }
                                    } else {
                                        bail!("Stack contains no elements");
                                    }
                                }
                            }
                            */

                            if !cmp_succeed {
                                continue;
                            }
                        }

                        transition_ret = Some(transition);
                        break;
                    }
                }
            }

            transition_ret
        }
        .ok_or(anyhow!("No suitable transition was found"))?;

        let popped = {
            if transition.pop_stack.is_some() {
                self.stack.pop()
            } else {
                None
            }
        };

        if let Some(actions) = &transition.action {
            for action in actions {
                match action {
                    Action::SwitchMode(mode) => self.mode = mode.clone(),
                    Action::Tree(action) => {
                        match action {
                            TreeAction::AddNode(node) => {
                                if let Some(value) = node {
                                    tree.add_node(value.clone(), line);
                                } else {
                                    unimplemented!("add node without defined value");
                                }
                            }
                            TreeAction::AppendChild(node) => {
                                if let Some(_) = node {
                                    unimplemented!("append child with defined value");
                                } else {
                                    tree.append_child(Node::Val(input.clone()), line)?;
                                }
                            }
                            TreeAction::GoUp => {
                                tree.go_up()?
                            }
                        }
                    },
                }
            }
        }

        if let Some(stack_type) = &transition.push_stack {
            if let Some(popped) = popped.filter(|p| *stack_type == p.to_stacktype()) {
                self.stack.push(popped);
            } else {
                self.stack.push(stack_type.to_stack_elem(input.clone())?);
            }
        }

        self.state = transition.to_state;

        if let Input::Any = transition.input {
            self.transition(input, next_token, tree, line)?;
        }

        Ok(())
    }
}