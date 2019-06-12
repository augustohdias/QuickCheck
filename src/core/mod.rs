#![allow(dead_code, unused_imports)]
pub mod arbitrary;

use self::arbitrary::Arbitrary;
use self::arbitrary::arbitrary;
use self::arbitrary::arbitrary_sized;

use std::fmt;

#[derive(Clone, Copy)]
struct TestConfig {
    pub cases: i32,
    pub size: i32,
    pub debug: bool
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
        QuickCheck {config: TestConfig {cases: 0, size: 0, debug: false}}
    }

    pub fn set_config(&mut self, _cases: i32, _size: i32) {
        self.config = TestConfig {cases: _cases, size: _size, debug: false};
    }

    pub fn set_size(&mut self, _size: i32) {
        self.config = TestConfig {
            cases: self.config.cases,
            size: _size,
            debug: self.config.debug
            };
    }

    pub fn set_cases(&mut self, _cases: i32) {
        self.config = TestConfig {
            cases: _cases,
            size: self.config.size,
            debug: self.config.debug
            };
    }

    pub fn set_debug(&mut self, _debug: bool) {
        self.config = TestConfig {
            cases: self.config.cases,
            size: self.config.size,
            debug: _debug
            };
    }

    pub fn run<T>(&self, property: &Fn(T) -> bool) where 
        T: Sized + fmt::Debug + Arbitrary + Clone {
        for _ in 0..self.config.cases {
            let test_case = arbitrary_sized::<T>(self.config.size as usize);
            
            if self.config.debug {
                println!("Current test case: {:?}", test_case);
            }

            if !property(test_case.clone()) {
                println!("Failed for the following test case: {:?}", test_case);
                return
            }
        }
        println!("All {} tests passed!", self.config.cases)
    }
}