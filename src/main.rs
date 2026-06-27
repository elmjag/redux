use redux::{input::*, sdl::*, store::*};

fn main() {
    let mut store = Store::new();

    process_sdl_event(&mut store, SdlEvent::new_key_down(0, KeyCode::Left));
    process_sdl_event(&mut store, SdlEvent::new_key_down(10, KeyCode::Right));
    process_sdl_event(&mut store, SdlEvent::new_key_up(20, KeyCode::Left));
    process_sdl_event(&mut store, SdlEvent::new_key_up(30, KeyCode::Right));

    println!("state {:?}", store.process(100));
}
