use crate::{
    actions::Action,
    sdl::{Event, KeyCode, SdlEvent},
    store::Store,
};

fn keycode_as_u32(keycode: KeyCode) -> u32 {
    match keycode {
        KeyCode::Left => 0,
        KeyCode::Right => 1,
    }
}

pub fn process_sdl_event(store: &mut Store, event: SdlEvent) {
    let timestamp = event.timestamp;

    let action = match event.event {
        Event::KeyDown(keycode) => Action::new_key_down(timestamp, keycode_as_u32(keycode)),
        Event::KeyUp(keycode) => Action::new_key_up(timestamp, keycode_as_u32(keycode)),
    };

    store.dispatch(action);
}
