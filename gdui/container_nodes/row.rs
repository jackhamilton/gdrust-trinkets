use godot::prelude::*;

use godot::classes::{Control, HBoxContainer};
use opencompose_rs::ast::OpenComposeAST;
use opencompose_rs::configs::View::ViewConfig;

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

pub struct ASTRowParser {}
impl ASTRowParser {
    pub fn parse_row(row_config: &ViewConfig, children: &OpenComposeAST) -> Gd<Control> {
        let mut row = HBoxContainer::new_alloc();
        let child_controls: Vec<Gd<Control>> = match children {
            OpenComposeAST::View(config, view_node) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(row_config);
                vec![ ASTParser::parse_view_node(&inherited_config, view_node) ]
            },
            OpenComposeAST::List(config, open_compose_asts) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(row_config);
                ASTParser::parse_list(&inherited_config, open_compose_asts)
            },
            OpenComposeAST::Container(config, container_node) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(row_config);
                vec![ ASTParser::parse_container_node(&inherited_config, container_node) ]
            },
        };
        for child in child_controls {
            row.add_child(&child);
        }
        row.upcast()
    }
}
