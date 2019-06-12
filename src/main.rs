#![allow(dead_code)]
mod core;

fn vec_sort(test_case: Vec<i32>) -> bool {
    let mut vec = test_case.clone();
    vec.sort();
    for i in 1..vec.len() {
        if vec.get(i-1) > vec.get(i) {
            return false
        }
    }
    true
}

fn vec_size_increase(test_case: (Vec<i32>, i32)) -> bool {
    let (mut vector, number) = test_case;
    let old_length = vector.len();
    vector.push(number);
    return vector.len() == old_length + 1
}

fn vec_contains(test_case: (Vec<i32>, i32)) -> bool {
    let (mut vector, number) = test_case;
    vector.push(number);
    vector.contains(&number)
}

fn vec_push(test_case: (Vec<i32>, i32)) -> bool {
    let (mut vector, number) = test_case;
    vector.push(number);
    match vector.last() {
        Some(n) => {
            if *n != number {
                return false
            }
            true
        }
        None => { 
            return false
        }
    }
}

fn vec_size_decrease(test_case: Vec<i32>) -> bool {
    let mut vector = test_case.clone();
    let old_length = vector.len();
    vector.pop();
    return vector.len() == (old_length - 1)
}

fn vec_remove(test_case: (Vec<i32>, i32)) -> bool {
    let (mut vector, seed) = test_case;
    let index = seed as usize % (vector.len() - 1);

    let (ok, element) = match vector.get(index + 1) {
        Some(element) => (true, *element),
        None => (false, 0)
    };

    if !ok {
        return false
    }
    
    vector.remove(index);
    return vector[index] == element
}

fn vec_multi_element_reverse(test_case: Vec<i32>) -> bool {
    let mut vector = test_case.clone();
    vector.reverse();
    vector.reverse();
    return vector == test_case
}

fn vec_single_element_reverse(test_case: Vec<i32>) -> bool {
    let mut vector = test_case.clone();
    vector.reverse();
    return vector == test_case
}

fn main () {
    let mut test = core::QuickCheck::new();
    test.set_config(100, 10);

    println!("Sorting test:");
    test.run::<Vec<i32>>(&vec_sort);
    
    println!("Size increase test:");
    test.run::<(Vec<i32>, i32)>(&vec_size_increase);
    
    println!("Content test:");
    test.run::<(Vec<i32>, i32)>(&vec_contains);
    
    println!("Remove test:");
    test.run::<(Vec<i32>, i32)>(&vec_remove);

    println!("Size decrease test:");
    test.run::<Vec<i32>>(&vec_size_decrease);

    println!("Reverse tests:");
    println!("* Multiple elements:");
    test.run::<Vec<i32>>(&vec_multi_element_reverse);

    println!("* One element:");
    test.set_size(1);
    test.run::<Vec<i32>>(&vec_single_element_reverse);


    
}