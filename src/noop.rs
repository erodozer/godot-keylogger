// under no-op implementation, match the base Godot key input
// depending on the OS, more than likely it'll only capture input when the window has focus

use godot::prelude::*;
use godot::classes::Engine;
use crate::keylogger::{Keylogger, SINGLETON_NAME, KeyloggerExtension};

#[gdextension]
unsafe impl ExtensionLibrary for KeyloggerExtension {

}
