use godot::classes::{Label, LabelSettings};
use godot::global::{HorizontalAlignment, VerticalAlignment};
use godot::prelude::*;

use godot::classes::{Control};
use opencompose_rs::configs::view_subtypes::view_alignment::Alignment;
use opencompose_rs::configs::Text::TextConfig;
use opencompose_rs::configs::View::ViewConfig;

pub struct ASTTextParser {}
impl ASTTextParser {
    pub fn parse_text(view_config: &ViewConfig, config: &TextConfig) -> Gd<Control> {
        let mut label = Label::new_alloc();
        label.set_text(config.text);
        let mut label_settings = LabelSettings::new_gd();
        label_settings.set_font_size(config.font_size.try_into().expect("Invalid label size"));
        if let Some(color) = view_config.foreground_color.clone() {
            label_settings.set_font_color(Color::from_rgba(color.r, color.g, color.b, color.a));
        }
        let horizontal = match config.horizontal_text_alignment {
            Alignment::Start => HorizontalAlignment::LEFT,
            Alignment::Center => HorizontalAlignment::CENTER,
            Alignment::End => HorizontalAlignment::RIGHT,
            Alignment::Fill => HorizontalAlignment::FILL,
        };
        let vertical = match config.vertical_text_alignment {
            Alignment::Start => VerticalAlignment::TOP,
            Alignment::Center => VerticalAlignment::CENTER,
            Alignment::End => VerticalAlignment::BOTTOM,
            Alignment::Fill => VerticalAlignment::FILL,
        };
        label.set_horizontal_alignment(horizontal);
        label.set_vertical_alignment(vertical);
        label.set_label_settings(&label_settings);
        label.upcast()
    }
}
