fn main() {
    // Closures have a superpower that functions don't. 
    // They can capture variables from their environment.
    // If you look closely, i is not a variable passed to or defined inside the closure.
    // i exists outside the closure and yet the closure can access it.
    // This ability to access variables from a scope outside of the closure's definition is 
    // called capturing from the environment. 
    let i = 42;
    let capture_i = || println!("{i}");
    capture_i();
    test();
    moving();
    ref_move();

    let mut pluralize_fox = create_pluralizer("fox".to_string());
    pluralize_fox();
    spawning_thread();
    call_fn();
    mutate_a_fn();
    expect_fnmut_but_pass_fn();
    expect_fnonce_but_pass_fnmut();
}

// Here the captured i is pushed into strings vec. This moves i out of the closure 
// which makes it implement FnOnce. 
// Even if you remove the move keyword, it is still FnOnce
// move keyword made no difference to which trait was implemented.
fn another_move_out_of_closure() {
    let i = "42".to_string();
    let mut strings = Vec::new();
    let capture_i: &dyn FnOnce() = &move || {
        strings.push(i); // move i out of the closure to the strings Vec
    };
}

// An important point to note is that the move keyword doesn't necessarily mean that 
// the closure will implement the FnOnce trait. 
// the following closure implements Fn even though move is used
// Which trait is implemented is decided by what the closure does with the captured variable, 
// not how it is captured
// move only forces the captured variable to be moved into the closure 
// while FnOnce is implemented if the closure moves the captured variable out.
fn move_in_fn() {
    let i = "42".to_string();
    let capture_i = move || {
        println!("{i}");
    };
}

fn expect_fnonce_but_pass_fnmut() {
    let a = 42;
    let mut i = "Rakudo".to_string();
    // This is an FnMut closure because it just mutates the captured i
    let fn_mut_closure = || {
        i.push_str(" Star");
    };
    // call_fnonce_closure(fn_mut_closure);
    call_fnonce_closure(|| println!("{a}"));
}

fn expect_fnmut_but_pass_fnonce() {
    let i = "Perl 6".to_string();
    call_fnmut_closure(|| {
        println!("Dropping {i}");
        // drop(i);
    })
}

// because Fn is a subtrait of FnMut. 
// Which means that all closures which implement Fn also implement FnMut.
fn expect_fnmut_but_pass_fn() {
    let i = 42;
    // This is an Fn closure because it doesn't mutate the captured i
    let fn_closure = || println!("Expect FnMut but got Fn: {i}");
    call_fnmut_closure(fn_closure);
}

fn mutate_a_fn() {
    let mut i = 42;
    let closure = || {
        i += 1;
        println!("{i}");
    };
    call_fnmut_closure(closure);
}

fn call_fn() {
    let i = 42;
    // Both capture_nothing and capture_i implement 'Fn'
    let capture_i = || println!("I capture i immutably: {i}");
    let capture_nothing = || println!("I capture nothing!");
    call_fn_closure(capture_i);
    call_fn_closure(capture_nothing);
}

fn call_fnonce_closure<T: FnOnce()>(t: T) {
    t();
}

fn call_fnmut_closure<T: FnMut()>(mut t: T) {
    t();
    t();
}

fn call_fn_closure<T: Fn()>(t: T) {
    t();
    t();
}

fn closure_eq() {
    let mut closure1 = || {};
    let closure2 = || {};
    // closure1 = closure2;
}

// A new thread takes a closure and if you want to force move some data to the new thread,
//  you can use the move keyword
fn spawning_thread() {
    let animal = "fox".to_string();
    std::thread::spawn(move || {
        println!("Animal owned by this thread: {animal}")})
      .join()
      .unwrap();  
}

// Because animal is moved into the create_pluralizer function, it is dropped at its end.
// So later when the returned closure is called it will access a dropped animal. 
fn create_pluralizer(mut animal: String) -> impl FnMut() {
    move || {
        animal.push_str("es"); // the closure captures animal by mutable borrow.
        println!("Pluralized animal: {animal}");
    }
}

fn ref_move() {
    let i: String = "42".to_string();
    let i_ref = &i;
    let capture_i = move || println!("{i_ref}");
    println!("{i}");
    println!("{i_ref}");
    capture_i();
}

fn move_keyword() {
    let i: String = "42".to_string();
    // added move before closure
    let capture_i = move || println!("Inside closure: {i}"); // take ownership of i
    // println!("Outside closure: {i}"); // comment out this line to compile
    capture_i();
}

fn moving() {
    let animal = "fox".to_string();
    let capture_animal = || {
        println!("Dropping {animal}");
        drop(animal);
    };
    
    capture_animal();
    // println!("Outside closure: {animal}");
}

fn borrowing_mutably() {
    let mut animal = "fox".to_string();
    let mut capture_animal = || {
        animal.push_str("es"); // mutable borrow occur here
    };
    capture_animal(); // mutable borrow ends here
    println!("Outside closure: {animal}");
}

fn test() {
    let i: String = "42".to_string();
    let capture_i = || println!("Inside closure: {i}");
    println!("Outside closure: {i}");
    capture_i();
}

fn foo() {
    let i = 42;
    fn bar() {
        // can't capture dynamic environment in a fn item
        // use the `|| { ... }` closure form instead
        // print!("{i}");
    }
}