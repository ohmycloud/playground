trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}

trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

impl Buildable<Bicycle, BicycleBuilder> for Bicycle {
    fn builder() -> BicycleBuilder {
        BicycleBuilder::new()
    }
}

impl Builder<Bicycle> for BicycleBuilder {
    fn new() -> Self {
        Self {
            bicycle: Bicycle {
                make: String::new(),
                model: String::new(),
                size: 0,
                color: String::new(),
            }
        }
    }
    fn build(self) -> Bicycle {
        self.bicycle
    }
}

#[derive(Debug)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color: String,
}

struct BicycleBuilder {
    bicycle: Bicycle,
}

macro_rules! with_str {
    ($name:ident, $func:ident) => {
        fn $func(&mut self, $name: &str) {
            self.bicycle.$name = $name.into();
        }
    };
}

macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        fn $func(&mut self, $name: $type) {
            self.bicycle.$name = $name
        }
    };
}

impl BicycleBuilder {
    with_str!(make, with_make);
    with_str!(model, with_model);
    with!(size, with_size, i32);
    with_str!(color, with_color);
}

fn main() {
    let mut builder = Bicycle::builder();
    builder.with_make("Huffy");
    builder.with_model("Radio");
    builder.with_size(46);

    let bicycle = builder.build();
    println!("{:?}", bicycle);
}