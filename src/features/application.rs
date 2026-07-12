use crate::{
    actions::{ActionVariant, BoxedActionVariant},
    dispatcher::Dispatcher,
    slices::{BoxedSlice, Slice},
    state::State,
};

pub struct QuitApplicationAction {}

impl ActionVariant for QuitApplicationAction {
    fn reduce(&self, timestamp: u32, state: &mut State, _dispatcher: &mut Dispatcher) {
        let app_slice = state.get_slice_mut::<ApplicationSlice>("application");
        app_slice.set_terminated(timestamp);
    }
}

impl QuitApplicationAction {
    pub fn new() -> BoxedActionVariant {
        Box::new(Self {})
    }
}

#[derive(Debug)]
pub enum AppState {
    Running,
    Terminated { termination_ts: u32 },
}

#[derive(Debug)]
pub struct ApplicationSlice {
    state: AppState,
}

impl Slice for ApplicationSlice {
    fn snapshot(&self, now: u32) -> BoxedSlice {
        let state = if let AppState::Terminated { termination_ts } = self.state
            && termination_ts <= now
        {
            AppState::Terminated { termination_ts }
        } else {
            AppState::Running
        };

        Box::new(Self { state })
    }
}

impl ApplicationSlice {
    pub fn new() -> BoxedSlice {
        Box::new(Self {
            state: AppState::Running,
        })
    }

    pub fn set_terminated(&mut self, termination_ts: u32) {
        self.state = AppState::Terminated { termination_ts };
    }

    pub fn is_terminated(&self) -> bool {
        matches!(self.state, AppState::Terminated { .. })
    }
}
