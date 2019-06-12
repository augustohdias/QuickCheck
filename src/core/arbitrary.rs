#![allow(dead_code, unused_imports, unused_variables)]
extern crate rand;

use std::iter;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp::Ord;
use self::rand::{Rng, thread_rng};
use self::rand::distributions::Alphanumeric;

pub trait Arbitrary {
	fn generate(sz: usize) -> Self;
}

pub fn arbitrary_sized<T: Arbitrary>(sz: usize) -> T {
  Arbitrary::generate(sz)
}

pub fn arbitrary<T: Arbitrary>() -> T {
  Arbitrary::generate(1)
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
                ($(arbitrary_sized::<$T>(sz)),+)
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
    arbitrary_sized::<String>(sz).into_boxed_str()
  }
}

impl<T: Arbitrary + Clone> Arbitrary for Vec<T> {
  fn generate(sz: usize) -> Vec<T> {
    let mut vector: Vec<T> = Vec::new();
    for _ in 0..sz {
      vector.push(arbitrary_sized::<T>(sz));
    }
    vector
  }
}

impl<T> Arbitrary for BTreeSet<T> 
  where T: Arbitrary + Ord + Clone 
{
  fn generate(sz: usize) -> BTreeSet<T> {
    let mut set: BTreeSet<T> = BTreeSet::new();
    for _ in 0..sz {
      set.insert(arbitrary::<T>());
    }
    set
  }
}

impl<T: Arbitrary + Clone> Arbitrary for Box<[T]> {
  fn generate(sz: usize) -> Box<[T]> {
    arbitrary_sized::<Vec<T>>(sz).into_boxed_slice()
  }
}
