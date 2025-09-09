use godot::prelude::*;
use godot::classes::Sprite2D;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
struct SceneManager {}

impl SceneManager {
    fn load_scene(path: String) {
        let main_scene = try_load::<PackedScene>(path).expect("Scene not found");
        let children = self.base_mut().get_tree().expect("No tree").get_root().expect("No root").get_children();
        self.base_mut().get_tree().expect("Tree not found").get_root().expect("No root").add_child(&main_scene.instantiate().expect("Failed to instantiate scene"));
        self.base_mut().get_tree().expect("Failed to get tree").set_pause(false);
        for mut child in children.iter_shared() {
            child.queue_free();
        }
    }
}
