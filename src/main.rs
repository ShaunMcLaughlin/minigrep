use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("The string is \"{}\" and the file path is \"{}\"",
        query, file_path);
    let contents = fs::read_to_string(file_path)
        .expect(format!("Failed to read {}", file_path).as_str());

    println!("With text:\n{contents}");

    let contents_vec: Vec<&str> = contents.split("\n").collect();
    println!("{contents_vec:#?}");
}
