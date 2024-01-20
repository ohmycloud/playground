fn trim_x(val: &String) -> &str {
    val.trim_matches('x')
}

fn main() {
    let val: String = "xxhelloxx".to_string();
    println!("val is '{}'", val);
    println!("trimmed is '{}'", trim_x(&val));
}
