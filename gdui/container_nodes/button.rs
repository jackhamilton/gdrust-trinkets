use godot::prelude::*;

use godot::classes::{Button, Control};
use opencompose_rs::ast::OpenComposeAST;
use opencompose_rs::configs::View::ViewConfig;

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

pub struct ASTButtonParser {}
impl ASTButtonParser {
    pub fn parse_button(box_config: &ViewConfig, children: &OpenComposeAST) -> Gd<Control> {
        let mut button = Button::new_alloc();
        let child_controls: Vec<Gd<Control>> = match children {
            OpenComposeAST::View(config, view_node) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(box_config);
                vec![ ASTParser::parse_view_node(&inherited_config, view_node) ]
            },
            OpenComposeAST::List(config, open_compose_asts) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(box_config);
                ASTParser::parse_list(&inherited_config, open_compose_asts)
            },
            OpenComposeAST::Container(config, container_node) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(box_config);
                vec![ ASTParser::parse_container_node(&inherited_config, container_node) ]
            },
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
