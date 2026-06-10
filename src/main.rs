mod actions;
mod reducer;
mod state;

use actions::*;
use reducer::*;
use state::*;

fn main() {
    let mut state = State::new();
    let mut snap = state.get_snapshot(0);

    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_rotate_left(0));
    snap = state.get_snapshot(5);
    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_stop(10));
    snap = state.get_snapshot(15);
    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_stop(15));
    snap = state.get_snapshot(20);
    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_rotate_right(20));
    snap = state.get_snapshot(25);
    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_stop(30));
    snap = state.get_snapshot(35);
    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_rotate_left(40));
    snap = state.get_snapshot(45);
    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_rotate_right(50));
    snap = state.get_snapshot(55);
    println!("    {state:?}\n--> {snap:?}\n");

    state = reduce(&state, &Action::new_stop(60));
    snap = state.get_snapshot(65);
    println!("    {state:?}\n--> {snap:?}\n");
}
