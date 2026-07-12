use crate::{
    actions::{Action, BoxedActionVariant},
    dispatcher::Dispatcher,
    slices::BoxedSlice,
    state::State,
};
use std::collections::BinaryHeap;

pub struct Store {
    state: State,
    pending_actions: BinaryHeap<Action>,
}

impl Store {
    pub fn new(slices: Vec<(&str, BoxedSlice)>) -> Self {
        let mut state = State::new();

        for (id, slice) in slices {
            state.add_slice(id, slice);
        }

        Self {
            state,
            pending_actions: BinaryHeap::new(),
        }
    }

    pub fn dispatch(&mut self, timestamp: u32, action: BoxedActionVariant) {
        self.pending_actions.push(Action::new(timestamp, action));
    }

    fn process_next_action(&mut self) {
        let mut dispatcher = Dispatcher::new();
        let next_action = self.pending_actions.pop().unwrap();

        next_action.reduce(&mut self.state, &mut dispatcher);

        for action in dispatcher {
            self.pending_actions.push(action);
        }
    }

    pub fn process(&mut self, timestamp: u32) -> State {
        while !self.pending_actions.is_empty() {
            self.process_next_action();
        }

        self.state.get_snapshot(timestamp)
    }
}
