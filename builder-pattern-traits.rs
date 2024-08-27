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

impl BicycleBuilder {
    fn with_make(&mut self, make: &str) -> &mut Self {
        self.bicycle.make = make.to_string();
        self
    }

    fn with_model(&mut self, model: &str) -> &mut Self {
        self.bicycle.model = model.to_string();
        self
    }

    fn with_size(&mut self, size: i32) -> &mut Self {
        self.bicycle.size = size;
        self
    }

    fn with_color(&mut self, color: &str) -> &mut Self {
        self.bicycle.color = color.to_string();
        self
    }
}

fn main() {
    let mut builder = Bicycle::builder();
    builder.with_make("Huffy");
    builder.with_model("Radio");
    builder.with_size(46);

    let bicycle = builder.build();
    println!("{:?}", bicycle);
}