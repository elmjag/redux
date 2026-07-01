use crate::{
    actions::{ActionVariant, BoxedActionVariant},
    dispatcher::Dispatcher,
    state::{self, RotationDirection, RotationState, State},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rotation {
    RotateLeft,
    RotateRight,
    Stop,
}

pub struct RotationAction {
    action: Rotation,
}

fn start_rotation(
    rotation: &state::Rotation,
    timestamp: u32,
    direction: RotationDirection,
) -> state::Rotation {
    let state = RotationState::Rotating {
        start_ts: timestamp,
        direction: direction,
    };

    match rotation.state() {
        RotationState::Idle => rotation.update(state, rotation.angle()),
        RotationState::Rotating {
            start_ts,
            direction,
        } => {
            let new_angle = rotation.get_updated_angle(direction, *start_ts, timestamp);
            rotation.update(state, new_angle)
        }
    }
}

fn stop_rotation(rotation: &state::Rotation, timestamp: u32) -> state::Rotation {
    match rotation.state() {
        RotationState::Idle => rotation.clone(),
        RotationState::Rotating {
            start_ts,
            direction,
        } => {
            let new_angle = rotation.get_updated_angle(direction, *start_ts, timestamp);
            rotation.update(RotationState::Idle, new_angle)
        }
    }
}

impl ActionVariant for RotationAction {
    fn reduce(&self, timestamp: u32, state: &State, _: &mut Dispatcher) -> State {
        let rotation = state.rotation();
        let new_rotation = match self.action {
            Rotation::RotateLeft => {
                start_rotation(rotation, timestamp, RotationDirection::Positive)
            }
            Rotation::RotateRight => {
                start_rotation(rotation, timestamp, RotationDirection::Negative)
            }
            Rotation::Stop => stop_rotation(rotation, timestamp),
        };

        state.update(new_rotation)
    }
}

impl RotationAction {
    pub fn new(direction: Rotation) -> BoxedActionVariant {
        Box::new(Self { action: direction })
    }
}
