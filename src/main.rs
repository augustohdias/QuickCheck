mod core;

fn sort_vec(v: &Vec<i32>) -> Vec<i32> {
    let mut vector = v.clone();
    vector.sort();
    vector
}

fn vec_sort_property(test_case: &Vec<i32>) -> bool {
    let sorted_vec = sort_vec(test_case);
    for i in 1..sorted_vec.len() {
        if sorted_vec.get(i-1) > sorted_vec.get(i) {
            return false
        }
    }
    true
}

fn vec_insertion_size_property(test_case: &(Vec<i32>, i32)) -> bool {
    let (immutable_vector, number) = test_case;
    let mut vector = immutable_vector.clone();
    let old_length = vector.len();
    vector.push(*number);
    if vector.len() != old_length + 1 {
        return false
    }
    true
}

fn vec_insertion_property(test_case: &(Vec<i32>, i32)) -> bool {
    let (immutable_vector, number) = test_case;
    let mut vector = immutable_vector.clone();
    vector.push(*number);
    vector.contains(number)
}

fn vec_removal_size_property(test_case: &Vec<i32>) -> bool {
    let mut vector = test_case.clone();
    let old_length = vector.len();
    vector.pop();
    if vector.len() != old_length - 1 {
        return false
    }
    true
}

fn main () {
    let mut test = core::QuickCheck::new();
    test.set_config(10, 100);
    test.run::<Vec<i32>>(&vec_sort_property);
    test.run::<(Vec<i32>, i32)>(&vec_insertion_property);
    test.run::<(Vec<i32>, i32)>(&vec_insertion_size_property);
    test.run::<Vec<i32>>(&vec_removal_size_property);
}