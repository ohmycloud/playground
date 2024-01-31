fn main() {
    // A borrowed value has a lifetime:
    // The lifetime can be implicit: add(p1: &Point, p2: &Point) -> Point
    // Lifetimes can also be explicit: &'a Point, &'document str
    // Read &'a Point as "a borrowed Point which is valid for at least the lifetime a"
    // Lifetimes are always inferred by the compiler: you cannot assign a lifetime yourself
    //   Lifetime annotations create constraints; the compiler verifies that there is a valid solution
    // Lifetimes for function arguments and return values must be fully specified, but Rust allows
    //  these to be elided in most case with a few simple rules.
    let p1: Point = Point(3, 4);
    let p2: Point = Point(6, 8);
    let p3: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);

    lifetime_in_struct();
    lifetime_in_function();

    let val: String = "xxhelloxx".to_string();
    println!("val is '{}'", val);
    println!("trimmed is '{}'", trim_x(&val));
}

#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

// Two references to two values are borrowed by a function and the function returns another reference.
// It must have come from one of those two inputs (or from a global variable).
// Which one is it? The compiler needs to to know,
// so at the call site the returned reference is not used for longer than a variable from where the reference came from.
fn demo() {
    let p1: Point = Point(3, 4);                // p1------'a
    let p3: &Point;                             // p3------'b
    
    {
        let p2: Point = Point(6, 8);           // p2-------'a
        p3 = lifetime_bounds(&p1, &p1);
    }
    println!("let-most point: {:?}", p3);
}

// Lifetime bounds can be applied to types or other lifetimes. The bound 'a: 'b is usually read as
// 'a outlives 'b. 'a: 'b means that 'a lasts at least as long as 'b, so a reference &'a () is valid whenever &'b () is
// valid.
fn lifetime_bounds<'a, 'b>(p1: &'a Point, p2: &'a Point) -> &'b Point
    where 'a: 'b {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

// T: 'a means that all lifetime parameters of T outlive 'a.
// For example, if 'a is an unconstrained lifetime parameter, then i32: 'static and &'static str: 'a are satisfied,
// but Vec<&'a ()>: 'static is not.
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
    y = x; // &'a i32 is a subtype of &'b i32, because 'a: 'b
    let r: &'b &'a i32 = &&0; // &'b &'a i32 is well formed because 'a: 'b
}

// If your data type stores borrowed data, it must annotated with a lifetime
// If text is consumed before the end of the lifetime of fox (or dog), the borrow checker throws an error.
// Types with borrowed data force users to hold on to the original data. 
//   This can be useful for creating lightweight views, but it generally makes them somewhat harder to use.
//   When possible, make data structures own their data directly.
// Some structs with multiple references inside can have more than one lifetime annotation.
// This can be necessary if there is a need to describe *lifetime relationships* between the references themselves,
// in addition to the lifetime of the struct itself. Those are very advanced use cases.
fn lifetime_in_struct() {
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let fox = Highlight(&text[4..19]);
    let dog: Highlight = Highlight(&text[35..43]); 
    // erase(text); // drop text, move `text` out of this function, into the erase function.
    println!("fox: {:?}", fox);
    println!("dog: {:?}", dog);
}

// the annotation on Highlight enforces that the data underlying the contained &str lives
// at least as long as any instance of Highlight that uses that data.
#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn lifetime_in_function() {
    let s1 = "raku";
    let s2 = "rakudo";
    let longest_str = longest(s1, s2);
    println!("longest str is : {}", longest_str);
}


fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() {
        s2
    } else {
        s1
    }
}

fn trim_x(val: &String) -> &str {
    val.trim_matches('x')
}
