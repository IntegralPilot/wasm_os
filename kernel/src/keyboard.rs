use crate::set_current_byte_in_stdin;
use alloc::{format, string::ToString};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub(crate) fn handle_keypress(scancode: u8) {
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => set_current_byte_in_stdin(character),
                DecodedKey::RawKey(key) => breadcrumbs::log!(
                    breadcrumbs::LogLevel::Warn,
                    "handle_keypresses",
                    format!("Unhandled RawKey: {:?}", key)
                ),
            }
        }
    }
}
