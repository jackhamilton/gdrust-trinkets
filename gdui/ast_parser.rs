use godot::prelude::*;

use godot::classes::Control;
use opencompose_rs::ast::{ContainerNode, OpenComposeAST, ViewNode};

use crate::gdrust_trinkets::gdui::view_nodes::{image::ASTImageParser, text::ASTTextParser};

pub struct ASTParser {
    pub ast: OpenComposeAST
}

impl ASTParser {
    pub fn parse(&self) -> Gd<Control> {
        match &self.ast {
            OpenComposeAST::View(view_node) => ASTParser::parse_view_node(&view_node),
            OpenComposeAST::List(_open_compose_asts) => panic!("Root node should not be a list"),
            OpenComposeAST::Container(container_node) => ASTParser::parse_container_node(&container_node),
        }
    }

    pub fn parse_list(nodes: &[OpenComposeAST]) -> Vec<Gd<Control>> {
        let mut controls = vec![];
        for node in nodes {
            let control = match node {
                OpenComposeAST::View(view_node) => ASTParser::parse_view_node(&view_node),
                OpenComposeAST::List(_open_compose_asts) => panic!("List should not contain a sublist!"),
                OpenComposeAST::Container(container_node) => ASTParser::parse_container_node(&container_node),
            };
            controls.push(control);
        }
        controls
    }

    pub fn parse_view_node(node: &ViewNode) -> Gd<Control> {
        match node {
            ViewNode::Image(image_config) => ASTImageParser::parse_image(image_config),
            ViewNode::Text(text_config) => ASTTextParser::parse_text(text_config),
        }
    }

    pub fn parse_container_node(node: &ContainerNode) -> Gd<Control> {
        match node {
            ContainerNode::Row(open_compose_ast) => todo!(),
            ContainerNode::Column(open_compose_ast) => todo!(),
            ContainerNode::Box(open_compose_ast) => todo!(),
            ContainerNode::Button(open_compose_ast) => todo!(),
        }

    }
}
