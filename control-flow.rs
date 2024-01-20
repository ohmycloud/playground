fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    contidion_must_be_bool();
    multi_if();
    let_if();
    if_and_else_have_incompatible_types();
}

fn contidion_must_be_bool() {
    let number = 3;

    // if 后面必须是 true 或 false 类型, 不会将整型等自动转换为 bool 类型!
    if number < 0  {
        println!("number was three");
    }
}

fn multi_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_if() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

// fails compile, `if` and `else` have incompatible types
fn if_and_else_have_incompatible_types() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}