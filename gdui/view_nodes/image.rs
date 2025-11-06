use godot::classes::Texture2D;
use godot::prelude::*;

use godot::classes::{Control, TextureRect};
use opencompose_rs::configs::Image::ImageConfig;
use opencompose_rs::configs::View::ViewConfig;

pub struct ASTImageParser {}
impl ASTImageParser {
    pub fn parse_image(view_config: &ViewConfig, config: &ImageConfig) -> Gd<Control> {
        let mut txrect = TextureRect::new_alloc();
        let texture = try_load::<Texture2D>(&config.url).expect("Texture not found");
        txrect.set_texture(&texture);
        txrect.upcast()
    }
}
