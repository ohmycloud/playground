// Both of these files are read at compile time.
const FILE_STR: &str = include_str!("main.rs");
const FILE_BYTES: &[u8] = include_bytes!("main.rs");

fn main() {
    // Output the content as string
    println!("{}", FILE_STR);
    // Output the content as byte array
    println!("{:?}", FILE_BYTES);
}
