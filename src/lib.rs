#![allow(dead_code)]

use nanoid::nanoid;
use chrono::Utc;
use tarantool::{error::Error, fiber, proc, space::Space, transaction::transaction};

#[proc]
fn insert() -> Result<bool, String> {
    let plan_item_space = Space::find("plan_item").ok_or_else(|| "Can't find space plan_item".to_string())?;

    let start_time = Utc::now().time();
    transaction(|| -> Result<(), Error> {
        for _ in 1..100_000 {
            let row = (nanoid!(), "title".to_string(), nanoid!());
            plan_item_space.replace(&row)?;
        }
        Ok(())
    })
    .unwrap();

    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run is {}", diff.num_milliseconds());

    fiber::sleep(std::time::Duration::from_millis(1));
    Ok(true)
}

#[proc]
fn fiber_async() -> Result<bool, String> {
    let mut f = fiber::Fiber::new("min_max", &mut |_: Box<()>| {
        let id = nanoid!();
        for step in 1..100 {
            println!("{:?} - {:?}", id, step);
            fiber::sleep(std::time::Duration::from_millis(100));
        }
        0
    });
    f.set_joinable(false);
    f.start(());
    // f.join();

    Ok(true)
}
