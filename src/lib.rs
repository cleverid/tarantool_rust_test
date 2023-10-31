#![allow(dead_code)]

mod domain;

use chrono::Utc;
use nanoid::nanoid;
use std::time::Duration;
use tarantool::{error::Error, fiber, proc, space::Space, transaction::transaction};

use crate::domain::Item;

#[proc]
fn insert() -> Result<bool, String> {
    let plan_item_space =
        Space::find("plan_item").ok_or_else(|| "Can't find space plan_item".to_string())?;

    let mut items: Vec<Item> = Vec::with_capacity(100_000);
    for _ in 1..items.capacity() {
        items.push(Item::random());
    }

    let start_time = Utc::now().time();
    for chunk in items.chunks(10) {
        transaction(|| -> Result<(), Error> {
            for item in chunk {
                plan_item_space.insert(&item.to_tuple())?;
            }
            Ok(())
        })
        .unwrap();
    }

    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run is {}", diff.num_milliseconds());
    Ok(true)
}

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
