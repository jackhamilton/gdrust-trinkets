use godot::prelude::*;

use godot::classes::{Control, VBoxContainer};
use opencompose_rs::ast::OpenComposeAST;

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

pub struct ASTColumnParser {}
impl ASTColumnParser {
    pub fn parse_column(children: &OpenComposeAST) -> Gd<Control> {
        let mut column = VBoxContainer::new_alloc();
        let child_controls: Vec<Gd<Control>> = match children {
            OpenComposeAST::View(view_node) => vec![ ASTParser::parse_view_node(view_node) ],
            OpenComposeAST::List(open_compose_asts) => ASTParser::parse_list(open_compose_asts),
            OpenComposeAST::Container(container_node) => vec![ ASTParser::parse_container_node(container_node) ],
        };
        for child in child_controls {
            column.add_child(&child);
        }
        column.upcast()
    }
}
