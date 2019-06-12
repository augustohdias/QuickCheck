#![allow(dead_code, unused_imports)]
pub mod arbitrary;

use self::arbitrary::Arbitrary;
use self::arbitrary::arbitrary;
use self::arbitrary::arbitrary_sized;

use std::fmt;

#[derive(Clone, Copy)]
struct TestConfig {
    pub cases: i32,
    pub size: i32
}

impl fmt::Debug for TestConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TestConfig {{ cases: {}, size: {} }}", self.cases, self.size)
    }
}

pub struct QuickCheck {
    config: TestConfig
}

impl QuickCheck {
    pub fn new() -> QuickCheck {
        QuickCheck {config: TestConfig {cases: 0, size: 0}}
    }

    pub fn set_config(&mut self, _cases: i32, _size: i32) {
        self.config = TestConfig {cases: _cases, size: _size};
    }
    
    pub fn run<T>(&self, property: &Fn(&T) -> bool) where 
        T: Sized + fmt::Debug + Arbitrary {
        for _ in 0..self.config.cases {
            let test_case = arbitrary_sized::<T>(self.config.size as usize);

            println!("Current test case: {:?}", test_case);

            if !property(&test_case) {
                println!("Failed for the following test case: {:?}", test_case);
                return
            }
        }
        println!("All {} tests passed!", self.config.cases)
    }
}