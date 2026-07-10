use crate::{
    actions::{ActionVariant, BoxedActionVariant},
    dispatcher::Dispatcher,
    features::rotation::{
        action::{RotationAction, RotationMotion},
        slice::{Direction, Motion, RotationSlice},
    },
    sdl::{Event as SdlEvent, KeyCode},
    state::State,
};

pub struct KeyInputAction {
    event: SdlEvent,
}

pub fn handle_key_down(dispatcher: &mut Dispatcher, timestamp: u32, keycode: &KeyCode) {
    let direction = match keycode {
        KeyCode::Left => RotationMotion::Left,
        KeyCode::Right => RotationMotion::Right,
    };
    dispatcher.dispatch(timestamp, RotationAction::new(direction));
}

pub fn handle_key_up(
    state: &mut State,
    dispatcher: &mut Dispatcher,
    timestamp: u32,
    keycode: &KeyCode,
) {
    let expected_dir = match keycode {
        KeyCode::Left => Direction::Positive,
        KeyCode::Right => Direction::Negative,
    };

    let rotation = state.get_slice::<RotationSlice>("rotation");

    if let Motion::Rotating { direction, .. } = rotation.motion()
        && direction == expected_dir
    {
        // only dispatch 'stop rotation' action, when
        // released key is the one the started rotation
        dispatcher.dispatch(timestamp, RotationAction::new(RotationMotion::Stop));
    }
}

impl ActionVariant for KeyInputAction {
    fn reduce(&self, timestamp: u32, state: &mut State, dispatcher: &mut Dispatcher) {
        match &self.event {
            SdlEvent::KeyDown(key_code) => handle_key_down(dispatcher, timestamp, key_code),
            SdlEvent::KeyUp(key_code) => handle_key_up(state, dispatcher, timestamp, key_code),
        }
    }
}

impl KeyInputAction {
    pub fn new(event: SdlEvent) -> BoxedActionVariant {
        Box::new(Self { event })
    }
}
