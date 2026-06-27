use redux::{
    actions::Action,
    state::{Rotation, RotationDirection, RotationState},
    store::Store,
};

fn is_close(left: f32, right: f32) -> bool {
    let diff = (left - right).abs();
    diff <= f32::EPSILON
}

fn assert_rotation(rotation: &Rotation, state: RotationState, angle: f32) {
    assert!(is_close(rotation.angle(), angle));
    assert_eq!(rotation.state(), &state);
}

#[test]
fn rotate_left() {
    let mut store = Store::new();

    store.dispatch(Action::new_rotate_left(0));
    let state = store.process(10);

    assert_rotation(
        &state.rotation(),
        RotationState::Rotating {
            start_ts: 0,
            direction: RotationDirection::Positive,
        },
        10.0,
    );
}

#[test]
fn key_down() {
    let mut store = Store::new();

    store.dispatch(Action::new_key_down(0, 1));
    let state = store.process(20);

    assert_rotation(
        &state.rotation(),
        RotationState::Rotating {
            start_ts: 0,
            direction: RotationDirection::Negative,
        },
        -20.0,
    );
}

#[test]
fn key_down_up() {
    let mut store = Store::new();

    store.dispatch(Action::new_key_down(0, 0));
    store.dispatch(Action::new_key_up(10, 0));
    let state = store.process(20);

    assert_rotation(&state.rotation(), RotationState::Idle, 10.0);
}

///
/// Test key events:
///   Left Down
///   Right Down
///   Left Up
///
#[test]
fn key_down_down_up() {
    let mut store = Store::new();

    store.dispatch(Action::new_key_down(0, 0));
    store.dispatch(Action::new_key_down(20, 1));
    // this should be ignored, we are rotating right (negative) direction)
    store.dispatch(Action::new_key_up(22, 0));

    let state = store.process(40);

    assert_rotation(
        &state.rotation(),
        RotationState::Rotating {
            start_ts: 20,
            direction: RotationDirection::Negative,
        },
        0.0,
    );
}
