use godot::classes::{EditorProperty, IEditorProperty};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, tool, internal, base = EditorProperty)]
pub struct EditorPropertySampleEditorProperty {
    base: Base<EditorProperty>,
}

#[godot_api]
impl IEditorProperty for EditorPropertySampleEditorProperty {
    fn update_property(&mut self) {
        let object = self.base().get_edited_object();

        // Uncommenting this will fix the crash when selecting a resource in the inspector
        // object.expect("None objects were filter by can_handle").call("reference", &[]);
    }
}
