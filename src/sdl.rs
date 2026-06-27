//! Faux SDL
#[derive(Debug)]
pub enum KeyCode {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Event {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
}

#[derive(Debug)]
pub struct SdlEvent {
    pub timestamp: u32,
    pub event: Event,
}

impl SdlEvent {
    fn new(timestamp: u32, event: Event) -> Self {
        Self { timestamp, event }
    }

    pub fn new_key_down(timestamp: u32, keycode: KeyCode) -> Self {
        Self::new(timestamp, Event::KeyDown(keycode))
    }

    pub fn new_key_up(timestamp: u32, keycode: KeyCode) -> Self {
        Self::new(timestamp, Event::KeyUp(keycode))
    }
}
