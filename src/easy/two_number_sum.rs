use std::collections::HashMap;
//create a map
//iterate over nums
//calculate compliment
//see if map contains compliment
//add current item to map

pub fn two_number_sum(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut nums_map: HashMap<i32, i32> = HashMap::new();
    for &number in nums {
        let complement = target - number;
        if nums_map.contains_key(&complement) {
            return Some((complement, number));
        }
        nums_map.insert(number, complement);
    }
    None
}

fn main() {
    let arr: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("{:?}", two_number_sum(&arr, 3));
}
