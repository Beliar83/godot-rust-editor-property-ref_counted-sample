mod editor_property_sample_editor_property;
mod editor_property_sample_inspector_plugin;
mod editor_property_sample_plugin;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
