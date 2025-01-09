fn bubble_sort(arr: &mut [isize]) {

    for j in 0..(arr.len() - 1) {
        for i in 0..(arr.len() - 1 - j) {
            if arr[i + 1] < arr[i] {
                arr.swap(i, i + 1);
            }
        }
    }
}

fn bubble_sort_copy(mut arr: Vec<i32>) -> Vec<i32> {

    for j in 0..(arr.len() - 1) {
        for i in 0..(arr.len() - 1 - j) {
            if arr[i + 1] < arr[i] {
                arr.swap(i, i + 1);
            }
        }
    }

    return arr;
}


fn main() {

    println!("Inplace sort");
    let mut arr = [5, 6, 1, 7, 1, 3];
    println!("Before: {:?}", arr);
    bubble_sort(&mut arr);
    println!("After: {:?}", arr);

    println!("Copy sort array");
    let arr = vec![5, 6, 1, 7, 1, 3];
    println!("Before: {:?}", arr);
    let sorted_array = bubble_sort_copy(arr);
    println!("After: {:?}", sorted_array);
}
