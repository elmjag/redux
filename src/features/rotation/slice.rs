use crate::slices::{BoxedSlice, Slice};

const RADIANS_PER_MS: f32 = 1.0;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Positive,
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Motion {
    Idle,
    Rotating { start_ts: u32, direction: Direction },
}

#[derive(Debug)]
pub struct RotationSlice {
    motion: Motion,
    angle: f32,
}

impl Slice for RotationSlice {
    fn snapshot(&self, timestamps: u32) -> BoxedSlice {
        let angle = if let Motion::Rotating {
            start_ts,
            direction,
        } = self.motion
        {
            self.get_updated_angle(direction, start_ts, timestamps)
        } else {
            self.angle
        };

        let snapshot = Self {
            motion: self.motion,
            angle,
        };

        Box::new(snapshot)
    }
}

impl RotationSlice {
    pub fn new() -> BoxedSlice {
        Box::new(Self {
            motion: Motion::Idle,
            angle: 0.0,
        })
    }

    pub fn motion(&self) -> Motion {
        self.motion
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    fn get_updated_angle(&self, direction: Direction, start_ts: u32, now_ts: u32) -> f32 {
        assert!(now_ts >= start_ts);
        let rotation_time = (now_ts - start_ts) as f32;
        let dir = match direction {
            Direction::Positive => 1.0,
            Direction::Negative => -1.0,
        };

        self.angle + rotation_time * dir * RADIANS_PER_MS
    }

    pub fn start_rotation(&mut self, timestamp: u32, direction: Direction) {
        self.angle = match self.motion {
            Motion::Idle => self.angle,
            Motion::Rotating {
                start_ts,
                direction,
            } => self.get_updated_angle(direction, start_ts, timestamp),
        };

        self.motion = Motion::Rotating {
            start_ts: timestamp,
            direction: direction,
        };
    }

    pub fn stop_rotation(&mut self, timestamp: u32) {
        self.angle = match self.motion {
            Motion::Idle => self.angle,
            Motion::Rotating {
                start_ts,
                direction,
            } => self.get_updated_angle(direction, start_ts, timestamp),
        };

        self.motion = Motion::Idle;
    }
}
