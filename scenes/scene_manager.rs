use std::{fmt::Display, path::PathBuf};

use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
struct SceneManager {
    #[export]
    pub scene_root_dir: GString,

    pub base: Base<Node>
}

#[godot_api]
impl SceneManager {
    /// Takes a full path. Ignores scene dir.
    #[func]
    fn load_scene(&mut self, path: String) {
        let main_scene = try_load::<PackedScene>(&path).expect("Scene not found");
        let children = self.base_mut().get_tree().expect("No tree").get_root().expect("No root").get_children();
        self.base_mut().get_tree().expect("Tree not found").get_root().expect("No root").add_child(&main_scene.instantiate().expect("Failed to instantiate scene"));
        self.base_mut().get_tree().expect("Failed to get tree").set_pause(false);
        for mut child in children.iter_shared() {
            child.queue_free();
        }
    }

    fn load_scene_key<T: Display>(&mut self, key: T) {
        let mut path: PathBuf = "".into();
        if !self.scene_root_dir.is_empty() {
            path.push(self.scene_root_dir.to_string());
        }
        path.push(key.to_string());
        let str_path = path.to_string_lossy().into_owned();
        self.load_scene(str_path);
    }

    #[func]
    fn new_with_root_dir(root_dir: String) {
        Gd::from_init_fn(|base| {
            Self {
                scene_root_dir: root_dir.into(),
                base
            }
        });
    }
}

#[godot_api]
impl INode for SceneManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            scene_root_dir: "".into(),
            base
        }
    }
}
