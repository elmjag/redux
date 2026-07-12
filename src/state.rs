use crate::slices::BoxedSlice;
use std::{any::Any, collections::HashMap};

#[derive(Debug)]
pub struct State {
    slices: HashMap<String, BoxedSlice>,
}

impl State {
    pub fn new() -> Self {
        Self {
            slices: HashMap::new(),
        }
    }

    pub fn add_slice(&mut self, id: &str, slice: BoxedSlice) {
        self.slices.insert(String::from(id), slice);
    }

    pub fn get_slice<T: Any>(&self, id: &str) -> &T {
        let boxed = self.slices.get(&String::from(id)).unwrap();
        let any: &dyn Any = boxed.as_ref();
        any.downcast_ref::<T>().unwrap()
    }

    pub fn get_slice_mut<T: Any>(&mut self, id: &str) -> &mut T {
        let boxed = self.slices.get_mut(&String::from(id)).unwrap();
        let any: &mut dyn Any = boxed.as_mut();
        any.downcast_mut::<T>().unwrap()
    }

    pub fn get_snapshot(&self, timestamp: u32) -> Self {
        let mut snapshot = State::new();

        for (id, slice) in self.slices.iter() {
            snapshot.add_slice(id, slice.snapshot(timestamp));
        }

        snapshot
    }
}
