fn single_number(list: Vec<u32>) -> u32 {
    let mut ones = 0u32;
    let mut twos = 0u32;
    let mut threes = 0u32;

    for i in list {
        twos |= ones & i;
        ones ^= i;
        threes = ones & twos;
        ones = ones ^ threes;
        twos = twos ^ threes;
    }

    ones
}

fn main() {
    //println!("Hello, world!");
    println!("{}", single_number(vec![1, 1, 1, 2, 2, 2, 4, 5, 5, 5]));
}
