use nanoid::nanoid;
use std::time::Duration;
use tarantool::{fiber, proc};

#[proc]
fn fiber_async() -> Result<bool, String> {
    let mut f = fiber::Fiber::new("min_max", &mut |_: Box<()>| {
        let id = nanoid!();
        let mut arr: Vec<usize> = Vec::with_capacity(1_000_000);
        for step in 1..arr.capacity() {
            arr.push(step);
            println!("{:?} - {:?}", id, step);
            fiber::sleep(Duration::from_micros(1));
            // fiber::fiber_yield();
        }
        println!("{:?} {:?}", id, arr.len());
        0
    });
    f.set_joinable(true);
    f.start(());
    // f.join();

    Ok(true)
}
