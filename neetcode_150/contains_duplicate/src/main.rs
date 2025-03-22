use std::collections::HashSet;

fn contains_duplicate(arr: &[isize]) -> bool {
    let mut seen = HashSet::new();
    return arr.iter().any(|&num| !seen.insert(num));
}

fn main() {
    let mut arr: Vec<isize> = (0..5).collect();
    arr.push(1);
    let val = contains_duplicate(&arr);
    println!("The array contains duplicate values: {}", val);
}
