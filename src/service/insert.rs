use chrono::Utc;
use tarantool::proc;

use crate::{domain::Item, repository::PlanItemRepository};

#[proc]
fn insert() -> Result<bool, String> {
    let items: Vec<Item> = (1..100_000).map(|_| Item::random()).collect();
    let start_time = Utc::now().time();

    let repo = PlanItemRepository::init().unwrap();
    for chunk in items.chunks(10) {
        repo.insert_chunk(chunk).unwrap();
    }

    let ms = (Utc::now().time() - start_time).num_milliseconds();
    println!("Time in ms: {:?}", ms);
    Ok(true)
}
