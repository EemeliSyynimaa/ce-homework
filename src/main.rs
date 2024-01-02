use std::fs;

fn main() {
    let path = "assets/part1/listing_0037_single_register_mov";
    let data: Vec<u8> = fs::read(path).unwrap();
    println!("Contents of file \"{}\":\n{:?}", path, data);
}
