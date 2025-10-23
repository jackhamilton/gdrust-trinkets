use godot::prelude::*;

use godot::classes::{Control, HBoxContainer};
use opencompose_rs::ast::OpenComposeAST;

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

pub struct ASTRowParser {}
impl ASTRowParser {
    pub fn parse_row(children: &OpenComposeAST) -> Gd<Control> {
        let mut row = HBoxContainer::new_alloc();
        let child_controls: Vec<Gd<Control>> = match children {
            OpenComposeAST::View(view_node) => vec![ ASTParser::parse_view_node(view_node) ],
            OpenComposeAST::List(open_compose_asts) => ASTParser::parse_list(open_compose_asts),
            OpenComposeAST::Container(container_node) => vec![ ASTParser::parse_container_node(container_node) ],
        };
        for child in child_controls {
            row.add_child(&child);
        }
        row.upcast()
    }
}
