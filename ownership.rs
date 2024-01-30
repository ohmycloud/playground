fn main() {
    scope_demo();
}

// The assignment of s1 to s2 transfers ownership
// The data was moved from s1, and s1 is no longer accessible
// When s1 goes out of scope, nothing happens: it has no ownership
// When s2 goes out of scope, the string data is freed.
// There is alaways exactly one variable binding which owns a value.
fn move_semantics() {
    // All assignment will transfer ownership between variables
    let s1 = String::from("Hello");
    let s2: String = s1;
    // println!("{}", s1);
}

fn scope_demo() {
    // At the end of the scope, the variable is dropped and the data is freed.
    // A destructor can run here to free up resources.
    // We say that the variable owns the value.
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1);
}


struct Point(i32, i32);
