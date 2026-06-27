use crate::{actions::Action, reducer::reduce, state::State};
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Store {
    state: State,
    pending_actions: BinaryHeap<Action>,
}

pub struct Dispatcher {
    actions: Vec<Action>,
}

impl Dispatcher {
    fn new() -> Self {
        Self { actions: vec![] }
    }

    pub fn dispatch(&mut self, action: Action) {
        self.actions.push(action);
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            pending_actions: BinaryHeap::new(),
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        self.pending_actions.push(action);
    }

    fn process_next_action(&mut self) {
        let mut dispatcher = Dispatcher::new();
        let next_action = self.pending_actions.pop().unwrap();

        self.state = reduce(&self.state, &next_action, &mut dispatcher);

        for action in dispatcher.actions {
            self.dispatch(action);
        }
    }

    pub fn process(&mut self, timestamp: u32) -> State {
        while !self.pending_actions.is_empty() {
            self.process_next_action();
        }

        self.state.get_snapshot(timestamp)
    }
}
