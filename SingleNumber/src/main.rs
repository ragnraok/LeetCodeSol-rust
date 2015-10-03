use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = match args[1].parse::<i32>() {
        Ok(ret) => ret,
        Err(_) => {
            println!("Error Number");
            panic!()
        }
    };
    let mut result: i32 = 0;
    for i in 0i32..num {
       result ^= i as i32;
    }
    println!("result: {}", result);
}
