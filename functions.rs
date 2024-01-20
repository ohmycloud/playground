fn main() {
    println!("Hello, world!");

    another_function(5);
    expression();

    let five = five();
    println!("give me five {} $", five);


    let cc = plus_one(5);
    println!("five plus one is {} ", cc);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1;     // 只有语句才在末尾加分号, 哼！
}