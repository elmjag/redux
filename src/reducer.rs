use crate::{
    actions::{Action, ActionKind},
    state::State,
};

pub fn reduce(state: &State, action: &Action) -> State {
    let ts = action.timestamp;

    match action.kind {
        ActionKind::RotateLeft => state.start_rotation_left(ts),
        ActionKind::RotateRight => state.start_rotation_right(ts),
        ActionKind::Stop => state.stop_rotation(ts),
    }
}
