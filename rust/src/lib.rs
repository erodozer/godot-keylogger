use godot::prelude::*;

mod linuxbsd;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {

}
