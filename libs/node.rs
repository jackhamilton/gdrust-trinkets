use godot::prelude::*;

trait HasTypedChildren {
    fn get_first_child<T: godot::prelude::Inherits<godot::prelude::Node>>(&self) -> Option<Gd<T>>;
    fn get_children<T: godot::prelude::Inherits<godot::prelude::Node>>(&self) -> Vec<Gd<T>>;
}

impl HasTypedChildren for Node {
    fn get_first_child<T: godot::prelude::Inherits<godot::prelude::Node>>(&self) -> Option<Gd<T>> {
        let binding = self.get_children();
        for child in binding.iter_shared() {
            let cast = child.clone().try_cast::<T>();
            if child.is_instance_valid() {
                match cast {
                    Ok(item) => return Some(item),
                    Err(_) => return None
                }
            }
        }
        None
    }

    fn get_children<T: godot::prelude::Inherits<godot::prelude::Node>>(&self) -> Vec<Gd<T>> {
        let binding = self.get_children();
        binding.iter_shared().filter_map(|child| {
            let cast = child.clone().try_cast::<T>();
            if child.is_instance_valid() {
                match cast {
                    Ok(item) => return Some(item),
                    Err(_) => return None
                }
            }
            None
        }).collect()
    }
}
