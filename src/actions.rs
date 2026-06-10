#[derive(Debug)]
pub enum ActionKind {
    RotateLeft,
    RotateRight,
    Stop,
}

#[derive(Debug)]
pub struct Action {
    pub timestamp: u32,
    pub kind: ActionKind,
}

impl Action {
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

    pub fn new_stop(timestamp: u32) -> Self {
        Self {
            timestamp,
            kind: ActionKind::Stop,
        }
    }
}
