const RADIANS_PER_MS: f32 = 1.0;

#[derive(Debug, Copy, Clone)]
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

    pub fn update(&self, new_state: RotationState, new_angle: f32) -> Rotation {
        Self {
            state: new_state,
            angle: new_angle,
        }
    }

    pub fn get_updated_angle(
        &self,
        direction: &RotationDirection,
        start_ts: u32,
        now_ts: u32,
    ) -> f32 {
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
}

impl State {
    pub fn new() -> Self {
        Self {
            rotation: Rotation::new(),
        }
    }

    pub fn update(&self, rotation: Rotation) -> Self {
        Self { rotation }
    }

    pub fn get_snapshot(&self, timestamp: u32) -> Self {
        Self {
            rotation: self.rotation.get_snapshot(timestamp),
        }
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }
}
