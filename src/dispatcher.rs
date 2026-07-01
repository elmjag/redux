use crate::actions::{Action, BoxedActionVariant};

pub struct Dispatcher {
    actions: Vec<Action>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }

    pub fn dispatch(&mut self, timestamp: u32, action: BoxedActionVariant) {
        self.actions.push(Action::new(timestamp, action));
    }
}

impl IntoIterator for Dispatcher {
    type Item = Action;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}
