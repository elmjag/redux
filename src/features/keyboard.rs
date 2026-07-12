use crate::{
    actions::{ActionVariant, BoxedActionVariant},
    dispatcher::Dispatcher,
    features::rotation::{
        action::{RotationAction, RotationMotion},
        slice::{Direction, Motion, RotationSlice},
    },
    state::State,
};

use sdl2::keyboard::Keycode;

pub enum KeyMotion {
    Up,
    Down,
}

pub struct KeyInputAction {
    motion: KeyMotion,
    key: Keycode,
}

impl ActionVariant for KeyInputAction {
    fn reduce(&self, timestamp: u32, state: &mut State, dispatcher: &mut Dispatcher) {
        match self.motion {
            KeyMotion::Down => handle_key_down(dispatcher, timestamp, self.key),
            KeyMotion::Up => handle_key_up(state, dispatcher, timestamp, self.key),
        }
    }
}

impl KeyInputAction {
    pub fn new(key: Keycode, motion: KeyMotion) -> BoxedActionVariant {
        Box::new(Self { key, motion })
    }
}

pub fn handle_key_down(dispatcher: &mut Dispatcher, timestamp: u32, key: Keycode) {
    let direction = match key {
        Keycode::Left => RotationMotion::Left,
        Keycode::Right => RotationMotion::Right,
        _ => panic!("unexpected keycode"),
    };
    dispatcher.dispatch(timestamp, RotationAction::new(direction));
}

pub fn handle_key_up(state: &mut State, dispatcher: &mut Dispatcher, timestamp: u32, key: Keycode) {
    let expected_dir = match key {
        Keycode::Left => Direction::Positive,
        Keycode::Right => Direction::Negative,
        _ => panic!("unexpected keycode"),
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
