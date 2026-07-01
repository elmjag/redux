use redux::{
    sdl::{Event as SdlEvent, KeyCode},
    slices::{
        keyboard::KeyInputAction,
        rotations::{self, RotationAction},
    },
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

    store.dispatch(0, RotationAction::new(rotations::Rotation::RotateLeft));
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

    store.dispatch(0, KeyInputAction::new(SdlEvent::KeyDown(KeyCode::Right)));

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

    store.dispatch(0, KeyInputAction::new(SdlEvent::KeyDown(KeyCode::Left)));
    store.dispatch(10, KeyInputAction::new(SdlEvent::KeyUp(KeyCode::Left)));

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

    store.dispatch(0, KeyInputAction::new(SdlEvent::KeyDown(KeyCode::Left)));
    store.dispatch(20, KeyInputAction::new(SdlEvent::KeyDown(KeyCode::Right)));

    // this should be ignored, we are rotating right (negative) direction)
    store.dispatch(22, KeyInputAction::new(SdlEvent::KeyUp(KeyCode::Left)));

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
