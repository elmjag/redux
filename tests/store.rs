use redux::{
    features::{
        keyboard::KeyInputAction,
        rotation::{
            action::{RotationAction, RotationMotion},
            slice::{Direction, Motion, RotationSlice},
        },
    },
    sdl::{Event as SdlEvent, KeyCode},
    store::Store,
};

fn is_close(left: f32, right: f32) -> bool {
    let diff = (left - right).abs();
    diff <= f32::EPSILON
}

fn assert_rotation(rotation: &RotationSlice, motion: Motion, angle: f32) {
    assert!(is_close(rotation.angle(), angle));
    assert_eq!(rotation.motion(), motion);
}

#[test]
fn rotate_left() {
    let mut store = Store::new();

    store.dispatch(0, RotationAction::new(RotationMotion::Left));
    let mut state = store.process(10);

    assert_rotation(
        state.get_slice("rotation"),
        Motion::Rotating {
            start_ts: 0,
            direction: Direction::Positive,
        },
        10.0,
    );
}

#[test]
fn key_down() {
    let mut store = Store::new();

    store.dispatch(0, KeyInputAction::new(SdlEvent::KeyDown(KeyCode::Right)));

    let mut state = store.process(20);

    assert_rotation(
        state.get_slice("rotation"),
        Motion::Rotating {
            start_ts: 0,
            direction: Direction::Negative,
        },
        -20.0,
    );
}

#[test]
fn key_down_up() {
    let mut store = Store::new();

    store.dispatch(0, KeyInputAction::new(SdlEvent::KeyDown(KeyCode::Left)));
    store.dispatch(10, KeyInputAction::new(SdlEvent::KeyUp(KeyCode::Left)));

    let mut state = store.process(20);

    assert_rotation(state.get_slice("rotation"), Motion::Idle, 10.0);
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

    let mut state = store.process(40);

    assert_rotation(
        state.get_slice("rotation"),
        Motion::Rotating {
            start_ts: 20,
            direction: Direction::Negative,
        },
        0.0,
    );
}
