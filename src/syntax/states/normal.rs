use crate::prelude::*;
use super::*;

impl PDA {
    pub fn add_normal_states(&mut self) {
        self.add_state(
            "q_exp_at",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_exp_keyword", Input::Token(TokenProto::At))
                    .action(vec![Action::Tree(TreeAction::AddNode(Some(Node::Id(Id::At))))])
                    .build(),
            ]
        );

        self.add_state(
            "q_exp_keyword",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_exp_colon", Input::Token(TokenProto::Path))
                    .action(vec![
                        Action::Tree(TreeAction::AddNode(Some(Node::Id(Id::Path)))),
                        Action::Tree(TreeAction::GoUp),
                    ])
                    .push_stack(StackType::Path)
                    .build(),

                TransitionBuilder::new("q_exp_colon", Input::Token(TokenProto::Class))
                    .action(vec![
                        Action::Tree(TreeAction::AddNode(Some(Node::Id(Id::Class)))),
                        Action::Tree(TreeAction::GoUp),
                    ])
                    .push_stack(StackType::Class)
                    .build(),

                TransitionBuilder::new("q_exp_colon", Input::Token(TokenProto::Method))
                    .action(vec![
                        Action::Tree(TreeAction::AddNode(Some(Node::Id(Id::Method)))),
                        Action::Tree(TreeAction::GoUp),
                    ])
                    .push_stack(StackType::Method)
                    .build(),
            ]
        );

        self.add_state(
            "q_exp_colon",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_exp_string", Input::Token(TokenProto::Colon))
                    .pop_stack(StackType::Path)
                    .push_stack(StackType::Path)
                    .build(),

                TransitionBuilder::new("q_exp_identifier", Input::Token(TokenProto::Colon))
                    .pop_stack(StackType::Class)
                    .push_stack(StackType::Class)
                    .build(),

                TransitionBuilder::new("q_exp_identifier", Input::Token(TokenProto::Colon))
                    .pop_stack(StackType::Method)
                    .push_stack(StackType::Method)
                    .build(),
            ]
        );

        self.add_state(
            "q_exp_string",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_exp_at", Input::Token(TokenProto::String))
                    .action(vec![
                        Action::Tree(TreeAction::AppendChild(None)),
                        Action::Tree(TreeAction::GoUp),
                    ])
                    .pop_stack(StackType::Path)
                    .build(),
            ]
        );

        self.add_state(
            "q_exp_identifier",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_exp_at", Input::Token(TokenProto::Identifier))
                    .action(vec![
                        Action::Tree(TreeAction::AppendChild(None)),
                        Action::Tree(TreeAction::GoUp),
                    ])
                    .pop_stack(StackType::Class)
                    .build(),

                TransitionBuilder::new("q_exp_lparen", Input::Token(TokenProto::Identifier))
                    .action(vec![
                        Action::Tree(TreeAction::AppendChild(None)),
                    ])
                    .pop_stack(StackType::Method)
                    .build(),
            ]
        );

        self.add_state(
            "q_exp_lparen",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_method_params", Input::Token(TokenProto::LParen))
                    .action(vec![Action::Tree(TreeAction::AddNode(Some(Node::Id(Id::Params))))])
                    .push_stack(StackType::MethodParams)
                    .build(),
            ]
        );

        self.add_state(
            "q_method_params",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_method_params_end", Input::Token(TokenProto::Identifier))
                    .action(vec![Action::Tree(TreeAction::AppendChild(None))])
                    .build(),
            ]
        );

        self.add_state(
            "q_method_params_end",
            ModeProto::Normal,
            vec![
                TransitionBuilder::new("q_method_params", Input::Token(TokenProto::Comma))
                    .build(),

                TransitionBuilder::new("q_exp_at", Input::Token(TokenProto::RParen))
                    .action(vec![
                        Action::Tree(TreeAction::GoUp),
                        Action::Tree(TreeAction::GoUp)
                    ])
                    .build(),
            ]
        );

        self.add_state(
            "q_accepting",
            ModeProto::Normal,
            vec![]
        );
    }
}