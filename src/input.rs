use crate::{features::keyboard::KeyInputAction, sdl::SdlEvent, store::Store};

pub fn process_sdl_event(store: &mut Store, event: SdlEvent) {
    store.dispatch(event.timestamp, KeyInputAction::new(event.event));
}
