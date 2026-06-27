use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum ActionKind {
    // key input actions
    KeyDown { keycode: u32 },
    KeyUp { keycode: u32 },

    // rotation change actions
    RotateLeft,
    RotateRight,
    StopRotation,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub timestamp: u32,
    pub kind: ActionKind,
}

impl Action {
    //
    // key input actions
    //
    pub fn new_key_down(timestamp: u32, keycode: u32) -> Self {
        Self {
            timestamp,
            kind: ActionKind::KeyDown { keycode },
        }
    }
    pub fn new_key_up(timestamp: u32, keycode: u32) -> Self {
        Self {
            timestamp,
            kind: ActionKind::KeyUp { keycode },
        }
    }

    //
    // rotation actions
    //
    pub fn new_rotate_left(timestamp: u32) -> Self {
        Self {
            timestamp,
            kind: ActionKind::RotateLeft,
        }
    }

    pub fn new_rotate_right(timestamp: u32) -> Self {
        Self {
            timestamp,
            kind: ActionKind::RotateRight,
        }
    }

    pub fn new_stop_rotation(timestamp: u32) -> Self {
        Self {
            timestamp,
            kind: ActionKind::StopRotation,
        }
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        // note, reversed order
        // we want actions with smallest timestamps first
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Action {}

impl PartialEq for Action {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
