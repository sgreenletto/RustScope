pub struct DemoItem {
    pub name: String,
    pub state: DemoState,
}

pub enum DemoState {
    Ready,
    Waiting,
    Failed,
}

pub trait Runnable {
    fn run(&self);
}

impl DemoItem {
    pub fn new(name: &str, state: DemoState) -> Self {
        Self {
            name: name.to_string(),
            state,
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, DemoState::Ready)
    }
}
