use std::collections::HashSet;

fn contain_duplicate(nums: Vec<u32>) -> bool {
    let mut nums_set:HashSet<u32> = HashSet::new();
    let nums_len = nums.len();
    for i in nums {
        nums_set.insert(i);
    }

    if nums_set.len() != nums_len {
        true
    } else {
        false
    }
}

fn main() {
    let input = vec![1u32, 2u32, 3u32];
    println!("{}", contain_duplicate(input));
}
