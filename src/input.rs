use crate::{
    features::application::QuitApplicationAction,
    features::keyboard::{KeyInputAction, KeyMotion},
    store::Store,
};
use sdl2::{EventPump, event::Event, keyboard::Keycode};

fn handle_key_event(event: Event, store: &mut Store) {
    let (timestamp, key, repeat, motion) = match event {
        Event::KeyDown {
            timestamp,
            keycode,
            repeat,
            ..
        } => (timestamp, keycode, repeat, KeyMotion::Down),
        Event::KeyUp {
            timestamp,
            keycode,
            repeat,
            ..
        } => (timestamp, keycode, repeat, KeyMotion::Up),
        _ => panic!("unexpected event"),
    };

    if repeat {
        return;
    }

    let key = key.unwrap();
    match key {
        Keycode::Left | Keycode::Right => {
            store.dispatch(timestamp, KeyInputAction::new(key, motion));
        }
        _ => {
            /* ignore other keys */
            return;
        }
    };
}

pub fn handle_event(event: Event, store: &mut Store) {
    match event {
        Event::KeyDown { .. } | Event::KeyUp { .. } => handle_key_event(event, store),
        Event::Quit { timestamp } => store.dispatch(timestamp, QuitApplicationAction::new()),
        _ => { /* ignore */ }
    }
}

pub fn pump_sdl_events(pump: &mut EventPump, store: &mut Store) {
    if let Some(event) = pump.wait_event_timeout(10) {
        handle_event(event, store);
    }
}
