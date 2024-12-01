use std::fs;

const INPUT_PATH: &str = "./input";

pub fn load_input(filename: &str) -> String {
    let path = format!("{}/{}", INPUT_PATH, filename);
    return fs::read_to_string(path).expect("Unable to read file");
}
