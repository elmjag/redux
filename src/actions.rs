use crate::{dispatcher::Dispatcher, state::State};
use std::cmp::Ordering;

pub trait ActionVariant {
    fn reduce(&self, timestamp: u32, state: &State, dispatcher: &mut Dispatcher) -> State;
}

pub type BoxedActionVariant = Box<dyn ActionVariant>;

pub struct Action {
    timestamp: u32,
    action: Box<dyn ActionVariant>,
}

impl Action {
    pub fn new(timestamp: u32, action: BoxedActionVariant) -> Self {
        Self { timestamp, action }
    }

    pub fn reduce(&self, state: &State, dispatcher: &mut Dispatcher) -> State {
        self.action.reduce(self.timestamp, state, dispatcher)
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        // note, reversed order
        // we want actions with smallest timestamps first
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Action {}

impl PartialEq for Action {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
