use chrono::Utc;
use nanoid::nanoid;

#[derive(Clone, Debug)]
pub struct Item {
    id: String,
    group_id: String,
    title: String,
    start_time: i64,
    end_time: i64,
}

#[rustfmt::skip]
//                id      group_id  title    add_time  update_time
pub type ItemTuple = (String, String,   String,  i64,      i64);

impl Item {
    pub fn random() -> Self {
        let id = nanoid!();
        let title = "Название работы".to_string();
        let group_id = nanoid!();
        let start_time = Utc::now().timestamp_micros();
        let end_time = start_time + 1000 * 60 * 60 * 24;
        Self {
            id,
            title,
            group_id,
            start_time,
            end_time,
        }
    }

    pub fn to_tuple(&self) -> ItemTuple {
        (
            self.id.clone(),
            self.group_id.clone(),
            self.title.clone(),
            self.start_time,
            self.end_time,
        )
    }
}
