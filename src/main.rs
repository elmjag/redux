use redux::{
    features::{application::ApplicationSlice, rotation::slice::RotationSlice},
    input::*,
    store::*,
};
use sdl2::video::Window;
use sdl2::{EventPump, TimerSubsystem};

fn init_sdl() -> (Window, EventPump, TimerSubsystem) {
    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context
        .video()
        .unwrap()
        .window("reduce this", 100, 100)
        .position_centered()
        .build()
        .unwrap();

    (
        window,
        sdl_context.event_pump().unwrap(),
        sdl_context.timer().unwrap(),
    )
}

fn main() {
    let (_window, mut event_pump, timer) = init_sdl();
    let mut store = Store::new(vec![
        ("rotation", RotationSlice::new()),
        ("application", ApplicationSlice::new()),
    ]);

    loop {
        pump_sdl_events(&mut event_pump, &mut store);

        let snap = store.process(timer.ticks());

        let app_state = snap.get_slice::<ApplicationSlice>("application");
        if app_state.is_terminated() {
            println!("bye");
            break;
        }

        let rot = snap.get_slice::<RotationSlice>("rotation");
        println!("{rot:?}");
    }
}
