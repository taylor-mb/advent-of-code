use std::fs;

fn main() {
    let text: String = fs::read_to_string("./hello_world.txt").expect("Something failed!");
    println!("{}", text);
}
