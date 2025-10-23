use godot::prelude::*;

use godot::classes::{Button, Control};
use opencompose_rs::ast::OpenComposeAST;

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

pub struct ASTButtonParser {}
impl ASTButtonParser {
    pub fn parse_button(children: &OpenComposeAST) -> Gd<Control> {
        let mut button = Button::new_alloc();
        let child_controls: Vec<Gd<Control>> = match children {
            OpenComposeAST::View(view_node) => vec![ ASTParser::parse_view_node(view_node) ],
            OpenComposeAST::List(open_compose_asts) => ASTParser::parse_list(open_compose_asts),
            OpenComposeAST::Container(container_node) => vec![ ASTParser::parse_container_node(container_node) ],
        };
        let mut z = -(TryInto::<i32>::try_into(child_controls.len()).expect("Could not convert usize to i32")) - 1;
        for mut child in child_controls {
            child.set_z_index(z);
            z += 1;
            button.add_child(&child);
        }
        button.upcast()
    }
}
