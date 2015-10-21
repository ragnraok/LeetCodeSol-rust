use std::collections::HashMap;

fn major_element(nums: Vec<i32>) -> i32 {
    let mut count_map:HashMap<i32, i32> = HashMap::new();
    let len = nums.len() as i32;
    for n in nums {
        *count_map.entry(n).or_insert(0) += 1;
    }

    for (n, count) in count_map {
        if count >= len / 2 {
            return n;
        }
    }
    -1
}

fn main() {
    println!("{}", major_element(vec![1, 2, 2, 3]));
}
