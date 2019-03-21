#![allow(dead_code)]
pub mod arbitrary;

use self::arbitrary::Arbitrary;
use self::arbitrary::arbitrary;

use std::fmt;

#[derive(Clone, Copy)]
pub struct TestParams {
    pub cases: i32,
    pub size: i32
}

impl TestParams {
    pub fn default_test() -> TestParams {
        TestParams {
            cases: 100, 
            size: 10
        }
    }
}

impl fmt::Debug for TestParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TestParams {{ cases: {}, size: {} }}", self.cases, self.size)
    }
}

pub fn run_test<T>(property: &Fn(&T) -> bool, &params: &TestParams) where 
    T: Sized + fmt::Debug + Arbitrary {
    for _ in 0..params.cases {
        let test_case = arbitrary::<T>(params.size as usize);

        println!("Current test case: {:?}", test_case);

        if !property(&test_case) {
            println!("Failed for the following test case: {:?}", test_case);
            return
        }
    }
    println!("All {} tests passed!", params.cases)
}