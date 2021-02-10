use std::io::Read as _;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let output = md_numbered_headers::process(&buffer);
    println!("{}", output);
}
