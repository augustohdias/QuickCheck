#![allow(dead_code)]
use std::fmt;

#[derive(Clone, Copy)]
pub struct TestInstance {
    cases: i32,
    size: i32
}

impl TestInstance {
    pub fn default_test() -> TestInstance {
        TestInstance {
            cases: 100, 
            size: 10
        }
    }
}

impl fmt::Debug for TestInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TestInstance {{ cases: {}, size: {} }}", self.cases, self.size)
    }
}