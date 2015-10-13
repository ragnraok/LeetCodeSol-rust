fn search_insert(list: Vec<u32>, target: u32) -> u32 {
    let mut index = 0u32;
    let mut result = 0;
    for elem in list {
        if elem == target {
            result = index;
            break;
        } else if elem > target {
            result = index;
            break;
        }
        index += 1;
    }
    result
}

fn main() {
    println!("{}", search_insert(vec![1, 3, 5, 6], 5));
}
