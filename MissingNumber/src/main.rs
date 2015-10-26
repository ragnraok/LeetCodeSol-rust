fn missing_number(nums: Vec<i32>) -> i32 {
    let len = nums.len() as i32;
    let mut sum: i32 = 0;
    for i in nums {
        sum += i;
    }
    let sum_n: i32 = len * (len + 1) / 2;
    sum_n - sum
}

fn main() {
    println!("{}", missing_number(vec![0, 1, 3]));
}
