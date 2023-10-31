use tarantool::{error::Error, space::Space, transaction::transaction};

use crate::domain::Item;

pub struct PlanItemRepository {
    space: Space,
}

impl PlanItemRepository {
    pub fn init() -> Result<Self, String> {
        let space = Space::find("plan_item").expect("Can't find space plan_item");
        Ok(Self { space })
    }

    pub fn insert_chunk(&self, chunk: &[Item]) -> Result<(), Error> {
        transaction(|| -> Result<(), Error> {
            for item in chunk {
                self.space.insert(&item.to_tuple())?;
            }
            Ok(())
        })
        .unwrap();
        Ok(())
    }
}
