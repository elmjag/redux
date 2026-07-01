use crate::{
    actions::{ActionVariant, BoxedActionVariant},
    dispatcher::Dispatcher,
    sdl::{Event as SdlEvent, KeyCode},
    slices::rotations::{Rotation, RotationAction},
    state::{RotationDirection, RotationState, State},
};

pub struct KeyInputAction {
    event: SdlEvent,
}

pub fn handle_key_down(dispatcher: &mut Dispatcher, timestamp: u32, keycode: &KeyCode) {
    let direction = match keycode {
        KeyCode::Left => Rotation::RotateLeft,
        KeyCode::Right => Rotation::RotateRight,
    };
    dispatcher.dispatch(timestamp, RotationAction::new(direction));
}

pub fn handle_key_up(
    state: &State,
    dispatcher: &mut Dispatcher,
    timestamp: u32,
    keycode: &KeyCode,
) {
    let expected_dir = match keycode {
        KeyCode::Left => RotationDirection::Positive,
        KeyCode::Right => RotationDirection::Negative,
    };

    if let RotationState::Rotating { direction, .. } = state.rotation().state()
        && direction == &expected_dir
    {
        // only dispatch 'stop rotation' action, when
        // released key is the one the started rotation
        dispatcher.dispatch(timestamp, RotationAction::new(Rotation::Stop));
    }
}

impl ActionVariant for KeyInputAction {
    fn reduce(&self, timestamp: u32, state: &State, dispatcher: &mut Dispatcher) -> State {
        match &self.event {
            SdlEvent::KeyDown(key_code) => handle_key_down(dispatcher, timestamp, key_code),
            SdlEvent::KeyUp(key_code) => handle_key_up(state, dispatcher, timestamp, key_code),
        }

        state.clone()
    }
}

impl KeyInputAction {
    pub fn new(event: SdlEvent) -> BoxedActionVariant {
        Box::new(Self { event })
    }
}
