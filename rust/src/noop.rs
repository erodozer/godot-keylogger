// under no-op implementation, match the base Godot key input
// depending on the OS, more than likely it'll only capture input when the window has focus

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Keylogger {
    base: Base<Node>,
}

#[godot_api]
impl Keylogger {

    #[func]
    fn is_key_pressed(&mut self, keycode: GKey) -> bool {
		return Input::singleton().is_key_pressed(keycode);
    }

    #[func]
    fn is_key_released(&mut self, keycode: GKey) -> bool {
		return Input::singleton().is_key_released(keycode);
    }

    #[func]
    fn is_key_just_pressed(&mut self, keycode: GKey) -> bool {
		return Input::singleton().is_key_just_pressed(keycode);
    }
}