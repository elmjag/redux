use crate::{
    actions::{ActionVariant, BoxedActionVariant},
    dispatcher::Dispatcher,
    features::rotation::slice::{Direction, RotationSlice},
    state::State,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RotationMotion {
    Left,
    Right,
    Stop,
}

pub struct RotationAction {
    action: RotationMotion,
}

impl ActionVariant for RotationAction {
    fn reduce(&self, timestamp: u32, state: &mut State, _: &mut Dispatcher) {
        let slice = state.get_slice::<RotationSlice>("rotation");
        match self.action {
            RotationMotion::Left => slice.start_rotation(timestamp, Direction::Positive),
            RotationMotion::Right => slice.start_rotation(timestamp, Direction::Negative),
            RotationMotion::Stop => slice.stop_rotation(timestamp),
        };
    }
}

impl RotationAction {
    pub fn new(direction: RotationMotion) -> BoxedActionVariant {
        Box::new(Self { action: direction })
    }
}
