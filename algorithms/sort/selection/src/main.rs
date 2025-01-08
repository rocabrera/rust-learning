fn selection_sort(arr: &mut [isize]) {

    for i in 0..arr.len() {
        let mut min = arr[i];
        let mut min_index = i;

        for (index, &elem) in arr.iter().enumerate().skip(i + 1) {
            if elem < min {
                min = elem;
                min_index = index;
            }
        }

        arr.swap(i, min_index);
    }
}

fn selection_sort_copya_arr(mut arr: [i32; 6]) -> [i32; 6] {
    let arr_size = arr.len();

    for i in 0..arr_size {
        let mut min = arr[i];
        let mut min_index = i;

        for (index, &elem) in arr.iter().enumerate().skip(i + 1) {
            if elem < min {
                min = elem;
                min_index = index;
            }
        }

        arr.swap(i, min_index);    
    }

    return arr;
}

fn selection_sort_copy_vec(mut arr: Vec<i32>) -> Vec<i32> {
    let arr_size = arr.len();

    for i in 0..arr_size {
        let mut min = arr[i];
        let mut min_index = i;

        for (index, &elem) in arr.iter().enumerate().skip(i + 1) {
            if elem < min {
                min = elem;
                min_index = index;
            }
        }

        arr.swap(i, min_index);    
    }

    return arr;
}

fn main() {
    println!("Inplace sort using arrays:");
    let mut arr = [64, 25, 12, 22, 11, -2];
    println!("Unsorted array: {:?}", arr);
    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);

    println!("Copy sort using Arrays:");
    let arr = [64, 25, 12, 22, 11, -2];
    println!("Unsorted array: {:?}", arr);
    let sorted_arr = selection_sort_copya_arr(arr);
    println!("Sorted array: {:?}", sorted_arr);

    println!("Copy sort using Vec:");
    let arr = vec![64, 25, 12, 22, 11, -2];
    println!("Unsorted array: {:?}", arr);
    let sorted_arr = selection_sort_copy_vec(arr);
    println!("Sorted array: {:?}", sorted_arr);

}
