const RADIANS_PER_MS: f32 = 2.0 / 1000.0;

#[derive(Debug)]
pub struct State {
    rotation: Rotation,
}

#[derive(Debug, Clone)]
enum RotationDirection {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
enum RotationState {
    Idle,
    Rotating {
        start_ts: u32,
        direction: RotationDirection,
    },
}

#[derive(Debug, Clone)]
struct Rotation {
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
