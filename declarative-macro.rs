trait Dog {
    fn name(&self) -> &String;
    fn age(&self) -> i32;
    fn breed(&self) -> &String;
}

macro_rules! print_what_it_is {
    () => {
        println!("nothing");
    };
    ($e:expr) => {
        println!("expression = {:?}", $e);
    };
    ($s:stmt) => {
        println!("statement = {:?}", $s);
    };
}

macro_rules! dog_struct {
    ($breed:ident) => {
        struct $breed {
            name: String,
            age: i32,
            breed: String,
        }
        impl $breed {
            fn new(name: &str, age: i32) -> Self {
                Self {
                    name: String::from(name),
                    age,
                    breed: stringify!($breed).into(),
                }
            }
        }
        impl Dog for $breed {
            fn name(&self) -> &String {
                &self.name

            }
            fn age(&self) -> i32 {
                self.age

            }
            fn breed(&self) -> &String {
                &self.breed
            }
        }
    };
}

fn main() {
    print_what_it_is!();
    dog_struct!(Poodle);
    dog_struct!(Golden);
    dog_struct!(Labrador);

    let peter = Poodle::new("Peter", 7);
    println!(
        "{} is a {} of age {}",
        peter.name(),
        peter.breed(),
        peter.age()
    );
}