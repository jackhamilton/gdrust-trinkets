use std::str::FromStr;

use crate::gdrust_trinkets::libs::singletons::TrinketSingletons;
use crate::gdrust_trinkets::persistence::storage_access::StorageAccessNode;
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct PersistentString {
    storage_access: Option<Gd<StorageAccessNode>>,
    #[export]
    pub key: GString,
    #[var]
    pub value: GString,

    pub base: Base<Node>
}

#[godot_api]
impl INode for PersistentString {
    fn init(base: Base<Node>) -> Self {
        Self {
            storage_access: None,
            key: "".into(),
            value: "".into(),
            base
        }
    }

    fn enter_tree(&mut self) {
        self.storage_access = Some(TrinketSingletons::get_storage());

        let storage = self.storage_access.as_ref().expect("No storage access found").bind();
        if self.key.is_empty() {
            panic!("Cannot load value for empty key");
        }
        self.value = GString::from_str(&storage.load_string(self.key.clone().into())).expect("Error converting to gstring");
    }

    fn exit_tree(&mut self) {
        let storage = self.storage_access.as_ref().expect("No storage access found").bind();
        if self.key.is_empty() {
            panic!("Cannot load value for empty key");
        }
        storage.save_string(self.key.clone().into(), self.value.clone().into());
    }
}
