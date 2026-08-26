use godot::prelude::*;
use std::collections::HashMap;
use godot::global::{Key as GKey, MouseButton};
use godot::classes::{DisplayServer, InputEvent, InputEventKey, InputEventMouseButton, InputEventMouseMotion};

#[derive(GodotClass)]
#[class(init, singleton, base=Node)]
pub struct GlobalInput {
    pub keystate: HashMap<godot::global::Key, bool>,
    pub prev_keystate: HashMap<godot::global::Key, bool>,
    pub hold_keystate: HashMap<godot::global::Key, bool>,

    pub mousestate: HashMap<godot::global::MouseButton, bool>,
    pub mouseposition: Vector2,
    
    pub penpressure: f32,

    pub base: Base<Node>,
}

#[godot_api]
impl GlobalInput {
    #[func]
    fn is_key_pressed(&self, keycode: GKey) -> bool {
        // map godot keycode to evdev keycode
        *self.keystate.get(&keycode).unwrap_or(&false)
    }

    #[func]
    fn is_key_released(&mut self, keycode: GKey) -> bool {
        !self.is_key_pressed(keycode)
    }

    #[func]
    fn is_key_just_pressed(&mut self, keycode: GKey) -> bool {
        // map godot keycode to evdev keycode
        match self.hold_keystate.get(&keycode) {
            Some(false) => self.is_key_pressed(keycode),
            _ => false
        }
    }

    #[func]
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        *self.mousestate.get(&button).unwrap_or(&false)
    }

    #[func]
    fn get_mouse_position(&self) -> Vector2 {
        self.mouseposition
    }

    #[func]
    fn get_pen_pressure(&self) -> f32 {
        self.penpressure
    }
}

#[godot_api]
impl INode for GlobalInput {
    fn ready(&mut self) {
        self.mouseposition = DisplayServer::singleton().mouse_get_position().cast_float() / DisplayServer::singleton().screen_get_size().cast_float();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
      if let Ok(keyevent) = event.clone().try_cast::<InputEventKey>() {
        let keycode = keyevent.get_keycode();

        if keyevent.is_pressed() {
          self.keystate.insert(keycode, true);
          self.hold_keystate.insert(keycode, true);
        }
        if keyevent.is_released() {
          self.keystate.insert(keycode, false);
        }
      }
      else if let Ok(motion) = event.clone().try_cast::<InputEventMouseMotion>() {
        self.mouseposition = motion.get_global_position();
      }
      else if let Ok(mouse) = event.clone().try_cast::<InputEventMouseButton>() {
        self.mousestate.insert(mouse.get_button_index(), mouse.is_pressed());
      }
    }

    fn process(&mut self, _delta: f64) {
      for (keycode, v) in self.hold_keystate.iter_mut() {
        let pressed = match self.keystate.get(keycode) {
            Some(res) => *res,
            _ => false
        };
        let prev_state = match self.prev_keystate.get(keycode) {
            Some(res) => *res,
            _ => false
        };

        *v = pressed && prev_state;
      }
    }
}

pub struct KeyloggerExtension;
