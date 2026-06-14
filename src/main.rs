mod actions;
mod reducer;
mod state;
mod store;

use actions::*;
use store::*;

fn main() {
    let mut store = Store::new();

    store.dispatch(Action::new_rotate_left(0));
    store.dispatch(Action::new_stop(10));
    store.dispatch(Action::new_stop(15));
    store.dispatch(Action::new_rotate_right(20));

    println!("{:?}", store.process(40));

    store.dispatch(Action::new_stop(40));
    store.dispatch(Action::new_rotate_left(50));
    store.dispatch(Action::new_rotate_right(60));
    store.dispatch(Action::new_stop(80));

    println!("{:?}", store.process(80));
    println!("{:?}", store.process(100));
}
