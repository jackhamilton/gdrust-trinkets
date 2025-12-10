use godot::prelude::*;

use crate::gdrust_trinkets::gdui::container_nodes::button::ASTButtonParser;
use crate::gdrust_trinkets::gdui::container_nodes::box_parser::ASTBoxParser;
use crate::gdrust_trinkets::gdui::container_nodes::column::ASTColumnParser;
use crate::gdrust_trinkets::gdui::container_nodes::row::ASTRowParser;
use crate::gdrust_trinkets::gdui::view_nodes::{image::ASTImageParser, text::ASTTextParser};

use opencompose_rs::ast::{ContainerNode, OpenComposeAST, ViewNode};
use opencompose_rs::configs::View::ViewSize;
use opencompose_rs::configs::View::ViewConfig;
use godot::classes::Control;

pub struct ASTParser {
    pub ast: OpenComposeAST
}

impl ASTParser {
    pub fn parse(&self) -> Gd<Control> {
        match &self.ast {
            OpenComposeAST::View(config, view_node) => ASTParser::parse_view_node(&config, &view_node),
            OpenComposeAST::List(_, _open_compose_asts) => panic!("Root node should not be a list"),
            OpenComposeAST::Container(config, container_node) => ASTParser::parse_container_node(&config, &container_node),
        }
    }

    pub fn parse_list(list_config: &ViewConfig, nodes: &[OpenComposeAST]) -> Vec<Gd<Control>> {
        let mut controls = vec![];
        for node in nodes {
            let control = match node {
                OpenComposeAST::View(config, view_node) => {
                    let mut inherited_config = config.clone();
                    inherited_config.inherit(list_config);
                    ASTParser::parse_view_node(&inherited_config, &view_node)
                },
                OpenComposeAST::List(_, _open_compose_asts) => panic!("List should not contain a sublist!"),
                OpenComposeAST::Container(config, container_node) => {
                    let mut inherited_config = config.clone();
                    inherited_config.inherit(list_config);
                    ASTParser::parse_container_node(&inherited_config, &container_node)
                }
            };
            controls.push(control);
        }
        controls
    }

    pub fn parse_view_node(node_config: &ViewConfig, node: &ViewNode) -> Gd<Control> {
        let final_node_config: Option<ViewConfig>;
        let mut control = match node {
            ViewNode::Image(config, image_config) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(node_config);
                final_node_config = Some(inherited_config.clone());
                ASTImageParser::parse_image(&inherited_config, image_config)
            },
            ViewNode::Text(config, text_config) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(node_config);
                final_node_config = Some(inherited_config.clone());
                ASTTextParser::parse_text(&inherited_config, text_config)
            },
        };
        Self::apply_control_config(&final_node_config.expect("Error: node config not carried"), &mut control);
        control
    }

    pub fn parse_container_node(node_config: &ViewConfig, node: &ContainerNode) -> Gd<Control> {
        let final_node_config: Option<ViewConfig>;
        let mut control = match node {
            ContainerNode::Row(config, open_compose_ast) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(node_config);
                final_node_config = Some(inherited_config.clone());
                ASTRowParser::parse_row(&inherited_config, open_compose_ast)
            },
            ContainerNode::Column(config, open_compose_ast) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(node_config);
                final_node_config = Some(inherited_config.clone());
                ASTColumnParser::parse_column(&inherited_config, open_compose_ast)
            },
            ContainerNode::Box(config, open_compose_ast) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(node_config);
                final_node_config = Some(inherited_config.clone());
                ASTBoxParser::parse_box(&inherited_config, open_compose_ast)
            },
            ContainerNode::Button(config, open_compose_ast) => {
                let mut inherited_config = config.clone();
                inherited_config.inherit(node_config);
                final_node_config = Some(inherited_config.clone());
                ASTButtonParser::parse_button(&inherited_config, open_compose_ast)
            },
        };
        Self::apply_control_config(&final_node_config.expect("Error: node config not carried"), &mut control);
        control
    }

    pub fn apply_control_config(node_config: &ViewConfig, control: &mut Gd<Control>) {
        let frame = &node_config.frame;
        // todo: eliminate clones
        match (frame.clone().width, frame.clone().height) {
            (ViewSize::Infinite, ViewSize::Finite(height)) => {
                let i16_height: i16 = height.try_into().expect("Error casting height");
                control.set_custom_minimum_size(Vector2 { x: 0.0, y: i16_height.into() });
                control.set_anchor(Side::LEFT, 0.0);
                control.set_anchor(Side::RIGHT, 1.0);
                godot_print!("applying frame: inf, {i16_height}");
            },
            (ViewSize::Finite(width), ViewSize::Finite(height)) => {
                let i16_width: i16 = width.try_into().expect("Error casting height");
                let i16_height: i16 = height.try_into().expect("Error casting height");
                control.set_custom_minimum_size(Vector2 { x: i16_width.into(), y: i16_height.into() });
                godot_print!("applying frame: {i16_width}, {i16_height}");
            },
            (ViewSize::Finite(width), ViewSize::Infinite) => {
                let i16_width: i16 = width.try_into().expect("Error casting height");
                control.set_custom_minimum_size(Vector2 { x: i16_width.into(), y: 0.0 });
                control.set_anchor(Side::TOP, 0.0);
                control.set_anchor(Side::BOTTOM, 1.0);
                godot_print!("applying frame: {i16_width}, inf");
            },
            (ViewSize::Infinite, ViewSize::Infinite) => {
                control.set_anchor(Side::LEFT, 0.0);
                control.set_anchor(Side::RIGHT, 1.0);
                control.set_anchor(Side::TOP, 0.0);
                control.set_anchor(Side::BOTTOM, 1.0);
            },
        }
    }
}
