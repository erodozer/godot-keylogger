use godot::prelude::*;

#[cfg(target_os = "windows")]
#[cfg(target_os = "macos")]
mod noop;

#[cfg(target_os = "linux")]
mod linuxbsd;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {

}

