mod analyzer;
mod domain;

use analyzer::Analyzer;
use domain::{DemoItem, DemoState, Runnable};

fn main() {
    let items = vec![
        DemoItem::new("alpha", DemoState::Ready),
        DemoItem::new("beta", DemoState::Waiting),
    ];
    let analyzer = Analyzer::new(items);
    analyzer.run();
}

pub fn run_demo(enabled: bool) {
    if enabled {
        for step in 0..3 {
            if step > 1 {
                println!("step {step}");
            }
        }
    } else {
        println!("disabled");
    }
}
