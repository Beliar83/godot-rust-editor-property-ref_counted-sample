use crate::editor_property_sample_inspector_plugin::EditorPropertySampleInspectorPlugin;
use godot::classes::{EditorPlugin, IEditorPlugin};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, tool, internal, base = EditorPlugin)]
pub struct EditorPropertySamplePlugin {
    base: Base<EditorPlugin>,
    inspector_plugin: Option<Gd<EditorPropertySampleInspectorPlugin>>,
}

#[godot_api]
impl IEditorPlugin for EditorPropertySamplePlugin {
    fn enter_tree(&mut self) {
        let inspector_plugin = EditorPropertySampleInspectorPlugin::new_gd();
        self.base_mut().add_inspector_plugin(&inspector_plugin);
        self.inspector_plugin = Some(inspector_plugin);
    }

    fn exit_tree(&mut self) {
        if let Some(plugin) = self.inspector_plugin.take() {
            self.base_mut().remove_inspector_plugin(&plugin);
            self.inspector_plugin = None;
        }
    }
}
