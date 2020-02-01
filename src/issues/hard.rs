// usually I dont understand if let statements
// there is a diagram as Match: if Let's big brother:
// http://patshaughnessy.net/2018/1/18/learning-rust-if-let-vs--match

// best example that I've prepared:
pub mod issues{
    pub fn iflet_issue() {
        let some_u8_value = None;
        let mut l: i32 = 0; // it has to be initialized
        if let Some(mut l) = some_u8_value {
            l = 3;
        }

        if let None = some_u8_value {
            l = 5;
        }
        println!("l : {}", l);
    }
}