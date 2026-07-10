use std::{any::Any, fmt::Debug};

pub trait Slice: Any + Debug {
    fn snapshot(&self, timestamps: u32) -> BoxedSlice;
}

pub type BoxedSlice = Box<dyn Slice>;
