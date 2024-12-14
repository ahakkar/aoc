use regex::Captures;

pub trait MyCaptures {
    fn get_i64(&self, i: usize) -> i64;
}

impl MyCaptures for Captures<'_> {
    fn get_i64(&self, i: usize) -> i64 {
        self.get(i)
            .expect("Capture group does not exist")
            .as_str()
            .parse::<i64>()
            .expect("Failed to parse capture group str to i64")
    }
}
