use crate::{
    actions::{Action, ActionKind},
    state::State,
    store::Dispatcher,
};

pub fn reduce(state: &State, action: &Action, dispatcher: &mut Dispatcher) -> State {
    let ts = action.timestamp;

    match action.kind {
        ActionKind::RotateLeft => state.start_rotation_left(ts),
        ActionKind::RotateRight => state.start_rotation_right(ts),
        ActionKind::StopRotation => state.stop_rotation(ts),
        ActionKind::KeyDown { keycode } => state.handle_key_down(dispatcher, ts, keycode),
        ActionKind::KeyUp { keycode } => state.handle_key_up(dispatcher, ts, keycode),
    }
}
