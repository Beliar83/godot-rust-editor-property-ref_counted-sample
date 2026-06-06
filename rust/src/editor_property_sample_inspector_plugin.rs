use crate::editor_property_sample_editor_property::EditorPropertySampleEditorProperty;
use godot::classes::{EditorInspectorPlugin, IEditorInspectorPlugin};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

#[derive(GodotClass)]
#[class(init, tool, internal, base = EditorInspectorPlugin)]
pub struct EditorPropertySampleInspectorPlugin {
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for EditorPropertySampleInspectorPlugin {
    fn can_handle(&self, object: Option<Gd<Object>>) -> bool {
        if let Some(object) = object
            && object.is_class(&RefCounted::class_id().to_gstring())
        {
            true
        } else {
            false
        }
    }

    fn parse_property(
        &mut self,
        object: Option<Gd<Object>>,
        _type_: VariantType,
        name: GString,
        _hint_type: PropertyHint,
        _hint_string: GString,
        _usage_flags: PropertyUsageFlags,
        _wide: bool,
    ) -> bool {
        let ref_counted = object
            .expect("None objects were filter by can_handle")
            .cast::<RefCounted>();
        godot_print!(
            "Instance: {}, Reference Count: {}",
            ref_counted.instance_id(),
            ref_counted.get_reference_count()
        );
        self.base_mut()
            .add_property_editor(&name, &EditorPropertySampleEditorProperty::new_alloc());
        false
    }
}
