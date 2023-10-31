use nanoid::nanoid;

#[derive(Clone, Debug)]
pub struct Dependency {
    predecessor_id: String,
    successor_id: String,
    dep_type: DepType,
    offset: i64,
}

#[derive(Copy, Clone, Debug)]
pub enum DepType {
    FF = 1,
    FS = 2,
    SF = 3,
    SS = 4,
}

#[rustfmt::skip]
//                      predecessor_id  successor_id    dep_type  offset
type DependencyTuple = (String,         String,         DepType,   i64);

impl Dependency {
    pub fn random() -> Self {
        Self {
            predecessor_id: nanoid!(),
            successor_id: nanoid!(),
            dep_type: DepType::FF,
            offset: 0,
        }
    }

    pub fn to_tuple(&self) -> DependencyTuple {
        (
            self.predecessor_id.clone(),
            self.successor_id.clone(),
            self.dep_type,
            self.offset,
        )
    }
}
