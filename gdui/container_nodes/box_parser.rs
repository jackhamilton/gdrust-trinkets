use godot::prelude::*;

use godot::classes::{Control, Container};
use opencompose_rs::ast::OpenComposeAST;

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

pub struct ASTBoxParser {}
impl ASTBoxParser {
    pub fn parse_box(children: &OpenComposeAST) -> Gd<Control> {
        let mut box_view = Container::new_alloc();
        let child_controls: Vec<Gd<Control>> = match children {
            OpenComposeAST::View(view_node) => vec![ ASTParser::parse_view_node(view_node) ],
            OpenComposeAST::List(open_compose_asts) => ASTParser::parse_list(open_compose_asts),
            OpenComposeAST::Container(container_node) => vec![ ASTParser::parse_container_node(container_node) ],
        };
        let mut z = 0;
        for mut child in child_controls {
            child.set_z_index(z);
            z += 1;
            box_view.add_child(&child);
        }
        box_view.upcast()
    }
}
