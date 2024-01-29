#![feature(exclusive_range_pattern)]

// 匹配字符串
fn match_string(input: &str) {
    match input {
        "bilbo"     => println!("Hello, Bilbo Baggins!"),
        "twostraws" => println!("Hello, Paul Hudson!"),
        _           => println!("身份验证失败"),
    }
}


// 匹配单个元组
fn match_tuple(input: (&str, &str)) {
    match input {
        ("bilbo", "bagg1n5")      => println!("Hello, Bilbo Baggins!"),
        ("twostraws", "fr0st1es") => println!("Hello, Paul Hudson!"),
        _                         => println!("你是谁?"),
    }
}

// 部分匹配, 只关心某些感兴趣的值, 不关心其它值
fn partial_match(input: (&str, &str, &str)) {
    match input {
        ("bilbo", "bagg1n5", _)      => println!("Hello, Bilbo Baggins!"),
        ("twostraws", "fr0st1es", _) => println!("Hello, Paul Hudson!"),
        _                            => println!("你是谁?"),
    }
}

// 只匹配元组的一部分
// 但仍想知道其它部分是什么
fn partial_match_known(input: (&str, &str)) {
    match input {
        ("bilbo", passwd)     => println!("Hello, Bilbo Baggins! {}", passwd),
        ("twostraws", passwd) => println!("Hello, Paul Hudson: your password was {}", passwd),
        _                     => println!("你是谁?"),
    }
}

fn fiaabuzz(input: i32) -> String {
    match (input % 3 == 0, input % 5 == 0) {
        (true, false)  => format!("Fizz"),
        (false, true)  => format!("Buzz"),
        (true, true)   => format!("FizzBuzz"),
        (false, false) => format!("{}", input),
    }
}

fn match_literal() {
    let twostraws = ("twostraws", "fr0st1es");
    let bilbo = ("bilbo", "bagg1n5");
    let taylor = ("taylor", "fr0st1es");
    let users = vec![twostraws, bilbo, taylor];

    for user in users {
        if let ("twostraws", "fr0st1es") = user {
            println!("User twostraws has the password {}", user.1);
        }
    }
}

fn match_range(input: u8) {
    match input {
        0 ..  18 => println!("你有活力有时间，但是没钱"),
        18 .. 70 => println!("你有活力有钱，但是没时间"),
        _        => println!("你有时间和金钱，但是没活力"),
    }
}

fn match_range_in_tuple(input: (&str, &str, u8)) {
    match input {
        (name, _, 0 .. 18)  => println!("{} 有活力有时间，但是没钱", name),
        (name, _, 18 .. 70) => println!("{} 有活力有钱，但是没时间", name),
        (name, _, _)        => println!("{} 有时间和金钱,但是没活力", name),
    }
}

pub enum Weather {
    Cloudy(u8),
    Sunny(u8),
    Windy(u8),
}

fn match_enum(weather: Weather) {
    match weather {
        Weather::Cloudy(humidness) => println!("{}", 4*humidness),
        Weather::Sunny(humidness)  => println!("{}", 2*humidness),
        Weather::Windy(humidness)  => println!("{}", 1*humidness),
    }
}

fn match_with_composite_condition() {
    let celebrities = vec!["Michael Jackson", "Taylor Swift", "MichaelCaine", "Adele Adkins", "Michael Jordan"];
    for celebrity in &celebrities {
        if celebrity.starts_with("Michael") {
            println!("{}", celebrity);
        }
    }

    for celebrity in &celebrities {
        if celebrity.chars().count() > 12 {
            println!("{}", celebrity);
        }
    }

    for celebrity in &celebrities {
        // 复合条件
        if celebrity.chars().count() > 12 && celebrity.starts_with("Michael") {
            println!("{}", celebrity);
        }
    }
}

fn multiple_alternatives(input: &str) {
    match input {
        "0" | "0x" | "0X" => println!("multiple alternatives"),
        _ => println!("something else"),
    }
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
}

fn match_when_destructure(persons: Vec<Person>) {
    let country = "USA".to_string();
    for Person {name, age, address} in persons {
        if let Address {street, city, country} = address {
            println!("name={}, age={}, country={:?}", name, age, country);
        }
    }
}

fn main() {
    let input = "twostraws";
    match_string(input);

    let input = ("twostraws", "fr0st1es");
    match_tuple(input);

    let input = ("twostraws", "fr0st1es", "127.0.0.1");
    partial_match(input);

    let input = ("twostraws", "fr0st1es");
    partial_match_known(input);

    let foo = fiaabuzz(15);
    println!("{}", foo);

    match_literal();
    match_range(17);

    let input = ("twostraws", "fr0st1es", 36);
    match_range_in_tuple(input);

    let input = Weather::Cloudy(1);
    match_enum(input);

    match_with_composite_condition();
    multiple_alternatives("0x");

    let alice = Person {
        name: "Alice".into(),
        age: 25,
        address: 
            Address {
                street: "1 Scala Lane".into(),
                city: "Chicago".into(),
                country: "USA".into(),
            }
    };

    let bob = Person {
        name: "Bob".into(),
        age: 29,
        address:
            Address {
                street: "2 Java Ave.".into(),
                city: "Miami".into(),
                country: "England".into(),
            }
    };
    
    let charlie = Person {
        name: "Charlie".into(),
        age: 32,
        address:
            Address {
                street: "3 Python Ct.".into(),
                city: "Boston".into(),
                country: "USA".into(),
            }
    };

    match_when_destructure(vec![alice, bob, charlie]);
}
