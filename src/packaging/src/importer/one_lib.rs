#[derive(Debug)]
pub struct Thing(i32);

impl Thing {
    pub fn new() -> Thing {
        Thing(4)
    }
}