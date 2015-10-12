fn plus_one(digits: &mut Vec<u32>) -> &Vec<u32> {
    let mut i = digits.len() - 1;
    while i >= 0 {
        if digits[i] + 1 == 10 {
            digits[i] = 0;
            if i > 0 {
                i -= 1;
            }
        } else {
            digits[i] += 1;
            break;
        }
    }

    //println!("i = {}", i);
    if i == 0 {
        digits[i] = 0;
    }

    if digits[0] == 0 {
        digits.insert(0, 1);
    }
    digits
}

fn main() {
    let mut vec1:Vec<u32> = vec![1, 2, 3];
    println!("{:?}", plus_one(&mut vec1));

    let mut vec2:Vec<u32> = vec![9, 8, 9];
    println!("{:?}", plus_one(&mut vec2));

    let mut vec3:Vec<u32> = vec![9, 9, 9];
    println!("{:?}", plus_one(&mut vec3));
}
