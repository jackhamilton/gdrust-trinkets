use godot::classes::IHSlider;
use godot::classes::HSlider;
use godot::{prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=HSlider)]
pub struct VolumeHSlider {
    #[export]
    pub persisted: bool,
    #[export]
    pub persistence_key: GString,
    pub base: Base<HSlider>
}

impl VolumeHSlider {}

#[godot_api]
impl IHSlider for VolumeHSlider {
    fn init(base: Base<HSlider>) -> Self {
        Self {
            persisted: false,
            persistence_key: "".into(),
            base
        }
    }

    fn enter_tree(&mut self) {

    }
}
