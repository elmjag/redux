use crate::{actions::Action, state::RotationState::Rotating, store::Dispatcher};

const RADIANS_PER_MS: f32 = 1.0;

#[derive(Debug)]
pub struct State {
    rotation: Rotation,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RotationDirection {
    Positive,
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RotationState {
    Idle,
    Rotating {
        start_ts: u32,
        direction: RotationDirection,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct Rotation {
    state: RotationState,
    angle: f32,
}

impl Rotation {
    fn new() -> Self {
        Self {
            state: RotationState::Idle,
            angle: 0.0,
        }
    }

    pub fn state(&self) -> &RotationState {
        &self.state
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    fn get_updated_angle(&self, direction: &RotationDirection, start_ts: u32, now_ts: u32) -> f32 {
        assert!(now_ts >= start_ts);
        let rotation_time = (now_ts - start_ts) as f32;
        let dir = match direction {
            RotationDirection::Positive => 1.0,
            RotationDirection::Negative => -1.0,
        };

        self.angle + rotation_time * dir * RADIANS_PER_MS
    }

    pub fn get_snapshot(&self, timestamp: u32) -> Self {
        let angle = match &self.state {
            RotationState::Idle => self.angle,
            RotationState::Rotating {
                start_ts,
                direction,
            } => self.get_updated_angle(&direction, *start_ts, timestamp),
        };

        Self {
            state: self.state.clone(),
            angle: angle,
        }
    }

    fn start_rotation_left(&self, timestamp: u32) -> Self {
        let state = RotationState::Rotating {
            start_ts: timestamp,
            direction: RotationDirection::Positive,
        };
        match &self.state {
            RotationState::Idle => Self {
                state: state,
                ..*self
            },
            RotationState::Rotating {
                start_ts,
                direction,
            } => Self {
                state: state,
                angle: self.get_updated_angle(direction, *start_ts, timestamp),
            },
        }
    }

    fn start_rotation_right(&self, timestamp: u32) -> Self {
        let state = RotationState::Rotating {
            start_ts: timestamp,
            direction: RotationDirection::Negative,
        };
        match &self.state {
            RotationState::Idle => Self {
                state: state,
                ..*self
            },
            RotationState::Rotating {
                start_ts,
                direction,
            } => Self {
                state: state,
                angle: self.get_updated_angle(direction, *start_ts, timestamp),
            },
        }
    }

    fn stop_rotation(&self, timestamp: u32) -> Self {
        match &self.state {
            RotationState::Idle => self.clone(),
            RotationState::Rotating {
                start_ts,
                direction,
            } => Self {
                state: RotationState::Idle,
                angle: self.get_updated_angle(direction, *start_ts, timestamp),
            },
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            rotation: Rotation::new(),
        }
    }

    pub fn get_snapshot(&self, timestamp: u32) -> Self {
        Self {
            rotation: self.rotation.get_snapshot(timestamp),
        }
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    pub fn handle_key_down(
        &self,
        dispatcher: &mut Dispatcher,
        timestamp: u32,
        keycode: u32,
    ) -> Self {
        if keycode == 0 {
            // left
            dispatcher.dispatch(Action::new_rotate_left(timestamp));
        } else {
            // right
            assert!(keycode == 1);
            dispatcher.dispatch(Action::new_rotate_right(timestamp));
        };
        Self { ..*self }
    }

    pub fn handle_key_up(&self, dispatcher: &mut Dispatcher, timestamp: u32, keycode: u32) -> Self {
        let expected_dir = if keycode == 0 {
            // left
            RotationDirection::Positive
        } else {
            // right
            assert!(keycode == 1);
            RotationDirection::Negative
        };

        if let Rotating { direction, .. } = self.rotation.state
            && direction == expected_dir
        {
            dispatcher.dispatch(Action::new_stop_rotation(timestamp));
        }

        Self { ..*self }
    }

    pub fn start_rotation_left(&self, timestamp: u32) -> Self {
        Self {
            rotation: self.rotation.start_rotation_left(timestamp),
        }
    }

    pub fn start_rotation_right(&self, timestamp: u32) -> Self {
        Self {
            rotation: self.rotation.start_rotation_right(timestamp),
        }
    }

    pub fn stop_rotation(&self, timestamp: u32) -> Self {
        Self {
            rotation: self.rotation.stop_rotation(timestamp),
        }
    }
}
