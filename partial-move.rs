fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    //  It creates two variables `name` and `age` and initializes them with `person.name` and `person.age`.
    //  Because of the `ref` the `age` variable will have the `type &Box<u8>` and point to person.age.
    let Person { name, ref age } = person;
    //           ---- value partially moved here

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person`
    // At this point the struct person is partially moved out, because of the move of person.name,
    // so you cannot use it anymore.
    println!("The person struct is {:?}", person);
    //                                    ^^^^^^ value borrowed here after partial move

    // But you can still use person.age because this was not moves out because of the ref
    println!("The person's age from person struct is {}", person.age);
}
