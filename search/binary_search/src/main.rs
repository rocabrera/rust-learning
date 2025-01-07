fn recursive_binary_search(arr: &[i32], left: i32, right: i32, number: i32) -> Option<usize> {

    let middle = ((left + right) / 2) as usize;
    
    if left > right {
        return None;
    }

    return match number.cmp(&arr[middle]) {
        std::cmp::Ordering::Equal => return Some(middle as usize),
        std::cmp::Ordering::Less => recursive_binary_search(arr, left, (middle as i32) - 1, number),
        std::cmp::Ordering::Greater => recursive_binary_search(arr, (middle as i32) +1, right, number),
    }
}

fn iterative_binary_search(arr: &[i32], mut left: i32, mut right: i32, number: i32) -> Option<usize> {

    while left <= right {
        let middle = ((left + right) / 2) as usize;
        match number.cmp(&arr[middle]) {
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Less => right = (middle as i32)  - 1,
            std::cmp::Ordering::Greater => left = (middle as i32) + 1,
        }
    }
    None
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, -2, 8];
    let arr  = [1, 2, 3, 4, 5, 6];
    let left_index: i32 = 0;
    let right_index: i32 = (arr.len() - 1) as i32;

    for &num in numbers.iter() {
        let _test = iterative_binary_search(&arr, left_index, right_index, num);
        println!("Iterative: {:?}", _test);
        let _test = recursive_binary_search(&arr, left_index, right_index, num);
        println!("Recursive: {:?}", _test);
    }

}
