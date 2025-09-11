use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
struct SceneManager {
    pub base: Base<Node>
}

impl SceneManager {
    fn load_scene(&mut self, path: String) {
        let main_scene = try_load::<PackedScene>(&path).expect("Scene not found");
        let children = self.base_mut().get_tree().expect("No tree").get_root().expect("No root").get_children();
        self.base_mut().get_tree().expect("Tree not found").get_root().expect("No root").add_child(&main_scene.instantiate().expect("Failed to instantiate scene"));
        self.base_mut().get_tree().expect("Failed to get tree").set_pause(false);
        for mut child in children.iter_shared() {
            child.queue_free();
        }
    }
}

#[godot_api]
impl INode for SceneManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base
        }
    }
}
