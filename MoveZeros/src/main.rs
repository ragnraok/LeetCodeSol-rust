fn move_zeros(nums: &mut Vec<i32>) {
    if nums.len() == 1 || nums.len() == 0 {
        return;
    }

    let mut zeroNum = 0;
    let len = nums.len();
    for i in 0..len {
        if nums[i] == 0 {
            zeroNum += 1;
        } else {
            nums[i - zeroNum] = nums[i];
        }
        println!("{:?}, i={}", nums, i);
    }

    println!("{:?}", nums);
    for i in 0..zeroNum {
        nums[len - zeroNum + 1] = 0;
    }
}

fn main() {
    let mut vec1 = vec![0, 1, 0, 3, 12];
    move_zeros(&mut vec1);
    println!("{:?}", vec1);
}
