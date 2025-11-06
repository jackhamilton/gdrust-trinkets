use godot::classes::{Label};
use godot::prelude::*;

use godot::classes::{Control};
use opencompose_rs::configs::Text::TextConfig;
use opencompose_rs::configs::View::ViewConfig;

pub struct ASTTextParser {}
impl ASTTextParser {
    pub fn parse_text(view_config: &ViewConfig, config: &TextConfig) -> Gd<Control> {
        let mut label = Label::new_alloc();
        label.set_text(config.text);
        label.upcast()
    }
}
