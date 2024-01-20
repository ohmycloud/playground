fn main() {
    let option_name: Option<String> = Some("Alice".to_owned());

    match option_name {
        Some(ref name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
    println!("{:?}", option_name);
}