fn main() {
    while_loops();
    for_loops();
    for_range();
    for_array();
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn for_array() {
    // Since 1.53
    let array = [1, 2, 3, 4, 5];

    for i in array { 
        println!("i = {}", i);
    }

    for i in array { 
        println!("i = {}", i);
    }
}