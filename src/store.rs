use crate::{actions::Action, reducer::reduce, state::State};

pub struct Store {
    state: State,
    pending_actions: Vec<Action>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            pending_actions: vec![],
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        self.pending_actions.push(action);
    }

    pub fn process(&mut self, timestamp: u32) -> State {
        for action in self.pending_actions.iter() {
            self.state = reduce(&self.state, action);
        }

        self.pending_actions.clear();
        self.state.get_snapshot(timestamp)
    }
}
