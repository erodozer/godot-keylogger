use godot::prelude::*;

use inputbot::KeybdKey;
use std::collections::HashSet;
use std::thread;

use strum::IntoEnumIterator;

use godot::global::Key as GKey;
use godot::classes::Os;

fn inputbot_to_godot(keycode: KeybdKey) -> Option<GKey> {
    match keycode {
        KeybdKey::Numrow0Key => Some(GKey::KEY_0),
        KeybdKey::Numrow1Key => Some(GKey::KEY_1),
        KeybdKey::Numrow2Key => Some(GKey::KEY_2),
        KeybdKey::Numrow3Key => Some(GKey::KEY_3),
        KeybdKey::Numrow4Key => Some(GKey::KEY_4),
        KeybdKey::Numrow5Key => Some(GKey::KEY_5),
        KeybdKey::Numrow6Key => Some(GKey::KEY_6),
        KeybdKey::Numrow7Key => Some(GKey::KEY_7),
        KeybdKey::Numrow8Key => Some(GKey::KEY_8),
        KeybdKey::Numrow9Key => Some(GKey::KEY_9),
        KeybdKey::AKey => Some(GKey::A),
        KeybdKey::BKey => Some(GKey::B),
        KeybdKey::CKey => Some(GKey::C),
        KeybdKey::DKey => Some(GKey::D),
        KeybdKey::FKey => Some(GKey::F),
        KeybdKey::EKey => Some(GKey::E),
        KeybdKey::GKey => Some(GKey::G),
        KeybdKey::HKey => Some(GKey::H),
        KeybdKey::IKey => Some(GKey::I),
        KeybdKey::JKey => Some(GKey::J),
        KeybdKey::KKey => Some(GKey::K),
        KeybdKey::LKey => Some(GKey::L),
        KeybdKey::MKey => Some(GKey::M),
        KeybdKey::NKey => Some(GKey::N),
        KeybdKey::OKey => Some(GKey::O),
        KeybdKey::PKey => Some(GKey::P),
        KeybdKey::QKey => Some(GKey::Q),
        KeybdKey::RKey => Some(GKey::R),
        KeybdKey::SKey => Some(GKey::S),
        KeybdKey::TKey => Some(GKey::T),
        KeybdKey::UKey => Some(GKey::U),
        KeybdKey::VKey => Some(GKey::V),
        KeybdKey::WKey => Some(GKey::W),
        KeybdKey::XKey => Some(GKey::X),
        KeybdKey::YKey => Some(GKey::Y),
        KeybdKey::ZKey => Some(GKey::Z),
        KeybdKey::BackslashKey => Some(GKey::BACKSLASH),
        KeybdKey::CommaKey => Some(GKey::COMMA),
        KeybdKey::SemicolonKey => Some(GKey::SEMICOLON),
        KeybdKey::QuoteKey => Some(GKey::APOSTROPHE),
        KeybdKey::BackquoteKey => Some(GKey::ASCIITILDE),
        KeybdKey::LBracketKey => Some(GKey::BRACELEFT),
        KeybdKey::RBracketKey => Some(GKey::BRACERIGHT),
        KeybdKey::SlashKey => Some(GKey::SLASH),
        KeybdKey::EqualKey => Some(GKey::EQUAL),
        // function keys
        KeybdKey::F1Key => Some(GKey::F1),
        KeybdKey::F2Key => Some(GKey::F2),
        KeybdKey::F3Key => Some(GKey::F3),
        KeybdKey::F4Key => Some(GKey::F4),
        KeybdKey::F5Key => Some(GKey::F5),
        KeybdKey::F6Key => Some(GKey::F6),
        KeybdKey::F7Key => Some(GKey::F7),
        KeybdKey::F8Key => Some(GKey::F8),
        KeybdKey::F9Key => Some(GKey::F9),
        KeybdKey::F10Key => Some(GKey::F10),
        KeybdKey::F11Key => Some(GKey::F11),
        KeybdKey::F12Key => Some(GKey::F12),
        // control buttons
        KeybdKey::EscapeKey => Some(GKey::ESCAPE),
        KeybdKey::TabKey => Some(GKey::TAB),
        KeybdKey::SpaceKey => Some(GKey::SPACE),
        KeybdKey::BackspaceKey => Some(GKey::BACKSPACE),
        KeybdKey::LControlKey => Some(GKey::CTRL),
        KeybdKey::RControlKey => Some(GKey::CTRL),
        KeybdKey::LAltKey => Some(GKey::ALT),
        KeybdKey::RAltKey => Some(GKey::ALT),
        KeybdKey::LShiftKey => Some(GKey::SHIFT),
        KeybdKey::RShiftKey => Some(GKey::SHIFT),
        KeybdKey::EnterKey => Some(GKey::ENTER),
        KeybdKey::LeftKey => Some(GKey::LEFT),
        KeybdKey::UpKey => Some(GKey::UP),
        KeybdKey::RightKey => Some(GKey::RIGHT),
        KeybdKey::DownKey => Some(GKey::DOWN),
        KeybdKey::PageDownKey => Some(GKey::PAGEDOWN),
        KeybdKey::PageUpKey => Some(GKey::PAGEUP),
        KeybdKey::InsertKey => Some(GKey::INSERT),
        KeybdKey::HomeKey => Some(GKey::HOME),
        KeybdKey::EndKey => Some(GKey::END),
        KeybdKey::CapsLockKey => Some(GKey::CAPSLOCK),
        KeybdKey::NumLockKey => Some(GKey::NUMLOCK),
        KeybdKey::ScrollLockKey => Some(GKey::SCROLLLOCK),
        // media buttons
        KeybdKey::MediaPlayPauseKey => Some(GKey::MEDIAPLAY),
        KeybdKey::MediaStopKey => Some(GKey::MEDIASTOP),
        KeybdKey::MediaNextTrackKey => Some(GKey::MEDIANEXT),
        KeybdKey::MediaPrevTrackKey => Some(GKey::MEDIAPREVIOUS),
        KeybdKey::VolumeUpKey => Some(GKey::VOLUMEUP),
        KeybdKey::VolumeDownKey => Some(GKey::VOLUMEDOWN),
        // keypad
        KeybdKey::Numpad0Key => Some(GKey::KP_0),
        KeybdKey::Numpad1Key => Some(GKey::KP_1),
        KeybdKey::Numpad2Key => Some(GKey::KP_2),
        KeybdKey::Numpad3Key => Some(GKey::KP_3),
        KeybdKey::Numpad4Key => Some(GKey::KP_4),
        KeybdKey::Numpad5Key => Some(GKey::KP_5),
        KeybdKey::Numpad6Key => Some(GKey::KP_6),
        KeybdKey::Numpad7Key => Some(GKey::KP_7),
        KeybdKey::Numpad8Key => Some(GKey::KP_8),
        KeybdKey::Numpad9Key => Some(GKey::KP_9),
        
        // TODO some buttons are missing from inputbot, instead expecting modifiers to map them
        _ => None
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Keylogger {
    keystate: HashSet<GKey>,
    holdstate: HashSet<GKey>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Keylogger {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            keystate: HashSet::new(),
            holdstate: HashSet::new(),
        }
    }

    fn ready(&mut self) {
        // create binder to all keys so global input states update
        KeybdKey::bind_all(|_| ());

        // prevent input handler from blocking main thread
        thread::spawn(|| inputbot::handle_input_events(false));
    }
    
    fn process(&mut self, _delta: f64) {
        self.keystate.clear();
        
        for key in KeybdKey::iter() {
            if let Some(godotkey) = inputbot_to_godot(key) {
                if key.is_pressed() {
                    self.holdstate.insert(godotkey);
                    self.keystate.insert(godotkey);
                } else {
                    self.holdstate.remove(&godotkey);
                }
            }
        }
    }
}

#[godot_api]
impl Keylogger {

    #[func]
    fn is_key_pressed(&mut self, keycode: GKey) -> bool {
        self.holdstate.contains(&keycode)
    }

    #[func]
    fn is_key_released(&mut self, keycode: GKey) -> bool {
        !self.is_key_pressed(keycode)
    }

    #[func]
    fn is_key_just_pressed(&mut self, keycode: GKey) -> bool {
        self.keystate.contains(&keycode)
    }
}


struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {

}
