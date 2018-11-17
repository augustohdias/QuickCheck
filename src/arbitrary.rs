#![allow(dead_code, unused_imports, unused_variables)]
extern crate rand;

use std::iter;
use self::rand::{Rng, thread_rng};
use self::rand::distributions::Alphanumeric;

pub trait Arbitrary {
	fn generate(sz: usize) -> Self;
}

pub fn arbitrary<T: Arbitrary>(sz: usize) -> T {
  Arbitrary::generate(sz)
}

macro_rules! arb_single_arg ( ($T:ty) => (
        impl Arbitrary for $T {
            fn generate(sz: usize) -> $T {
                rand::random()
            }
        }
    )
);

macro_rules! arb_multi_arg ( ($($T:ident),+ ) => (
        impl<$($T: Arbitrary),+> Arbitrary for ($($T),+) {
            fn generate(sz: usize) -> ($($T),+) {
                ($(arbitrary::<$T>(sz)),+)
            }
        }
    )
);

arb_single_arg!(u8);
arb_single_arg!(i8);
arb_single_arg!(i32);
arb_single_arg!(i64);
arb_single_arg!(f32);
arb_single_arg!(f64);
arb_single_arg!(bool);
arb_single_arg!(());

arb_multi_arg!(A, B);
arb_multi_arg!(A, B, C);
arb_multi_arg!(A, B, C, D);
arb_multi_arg!(A, B, C, D, E);
arb_multi_arg!(A, B, C, D, E, F);
arb_multi_arg!(A, B, C, D, E, F, G);
arb_multi_arg!(A, B, C, D, E, F, G, H);
arb_multi_arg!(A, B, C, D, E, F, G, H, I);
arb_multi_arg!(A, B, C, D, E, F, G, H, I, J);

impl Arbitrary for char {
  fn generate(_: usize) -> char {
    rand::random::<u8>() as char
  }
}

impl Arbitrary for String {
  fn generate(sz: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(sz)
        .collect()
  }
}

impl Arbitrary for Box<str> {
  fn generate(sz: usize) -> Box<str> {
    arbitrary::<String>(sz).into_boxed_str()
  }
}

impl<T: Arbitrary + Clone> Arbitrary for Vec<T> {
  fn generate(sz: usize) -> Vec<T> {
    vec![arbitrary::<T>(sz); sz]
  }
}

impl<T: Arbitrary + Clone> Arbitrary for Box<[T]> {
  fn generate(sz: usize) -> Box<[T]> {
    arbitrary::<Vec<T>>(sz).into_boxed_slice()
  }
}
