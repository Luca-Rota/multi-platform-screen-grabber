use chrono::Utc;
use iced::{Event};
use iced::keyboard;
use chrono::{Datelike, Timelike};

pub fn check_shortcut_event(event: &Event) -> String {
    if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, modifiers }) = event {
        if modifiers.control() || modifiers.alt() || modifiers.shift() {
            if let Some(character) = get_character_from_keycode(*key_code) {
                if character.is_ascii_alphabetic() {
                    let formatted_shortcut = format!("{:#?} + {}", modifiers, character);
                    return formatted_shortcut;
                }
            }
        }
    }
    return "".to_string();
}

pub fn get_character_from_keycode(key_code: keyboard::KeyCode) -> Option<char> {
    match key_code {
        keyboard::KeyCode::A => Some('a'),
        keyboard::KeyCode::B => Some('b'),
        keyboard::KeyCode::C => Some('c'),
        keyboard::KeyCode::D => Some('d'),
        keyboard::KeyCode::E => Some('e'),
        keyboard::KeyCode::F => Some('f'),
        keyboard::KeyCode::G => Some('g'),
        keyboard::KeyCode::H => Some('h'),
        keyboard::KeyCode::I => Some('i'),
        keyboard::KeyCode::J => Some('j'),
        keyboard::KeyCode::K => Some('k'),
        keyboard::KeyCode::L => Some('l'),
        keyboard::KeyCode::M => Some('m'),
        keyboard::KeyCode::N => Some('n'),
        keyboard::KeyCode::O => Some('o'),
        keyboard::KeyCode::P => Some('p'),
        keyboard::KeyCode::Q => Some('q'),
        keyboard::KeyCode::R => Some('r'),
        keyboard::KeyCode::S => Some('s'),
        keyboard::KeyCode::T => Some('t'),
        keyboard::KeyCode::U => Some('u'),
        keyboard::KeyCode::V => Some('v'),
        keyboard::KeyCode::W => Some('w'),
        keyboard::KeyCode::X => Some('x'),
        keyboard::KeyCode::Y => Some('y'),
        keyboard::KeyCode::Z => Some('z'),
        _ => None,
    }
}

pub fn generate_current_time_string() -> String {
    let current_time = Utc::now();
    format!(
        "Screenshot_{:04}_{:02}_{:02}_{:02}_{:02}_{:02}",
        current_time.year(),
        current_time.month(),
        current_time.day(),
        current_time.hour(),
        current_time.minute(),
        current_time.second()
    )
}
