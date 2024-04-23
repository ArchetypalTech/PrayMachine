fn main() {
    let path = std::env::args().nth(1).expect("path to config file");
    println!("path: {:?}", path);
}