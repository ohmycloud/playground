// cannot return value referencing local variable `v`
fn f1<'a>() -> Vec<&'a String> {
    let mut vec = Vec::new();
    let v = String::from("local");
    vec.push(&v);
    
    return vec;
}

fn main() {
    let v = f1();
    println!("{:?}", v);
}
