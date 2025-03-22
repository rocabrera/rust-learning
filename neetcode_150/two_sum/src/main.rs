// use std::collections::HashMap;

// fn two_sum(nums: Vec<i32>, target: i32) -> (usize, usize) {
    
//     let mut positions: HashMap<i32, usize> = HashMap::new();

//     for (index, num) in nums.iter().enumerate() {
    
//         let complement = target - num;
//         if positions.contains_key(&complement){
//             return (*positions.get(&complement).unwrap(), index);
//         }
//         else {
//             positions.insert(*num, index);
//         }
//     }

//     return (0, 0);
// }

// fn main() {
//     let nums = vec![2, 7, 11, 15];
//     let target = 2+11;
//     let tuple = two_sum(nums, target);
//     println!("{} {}", tuple.0, tuple.1);
// }

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut positions: HashMap<i32, usize> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
        
            let complement = target - num;
            if positions.contains_key(&complement){
                return vec![*positions.get(&complement).unwrap() as i32, index as i32];
            }
            else {
                positions.insert(*num, index);
            }
        }
        return vec![0,0];
    }
}