use crate::{sdl::SdlEvent, slices::keyboard::KeyInputAction, store::Store};

pub fn process_sdl_event(store: &mut Store, event: SdlEvent) {
    store.dispatch(event.timestamp, KeyInputAction::new(event.event));
}
