use crate::domain::{DemoItem, DemoState, Runnable};

pub struct Analyzer {
    items: Vec<DemoItem>,
}

impl Analyzer {
    pub fn new(items: Vec<DemoItem>) -> Self {
        Self { items }
    }

    pub fn analyze(&self) -> Result<usize, String> {
        let mut ready = 0;
        let mut index = 0;

        while index < self.items.len() {
            let item = &self.items[index];
            match item.state {
                DemoState::Ready => {
                    ready += 1;
                }
                DemoState::Waiting => {
                    if item.name.is_empty() {
                        return Err("missing name".to_string());
                    }
                }
                DemoState::Failed => loop {
                    break;
                },
            }
            index += 1;
        }

        Ok(ready)
    }
}

impl Runnable for Analyzer {
    fn run(&self) {
        match self.analyze() {
            Ok(count) => println!("ready: {count}"),
            Err(error) => println!("error: {error}"),
        }
    }
}
