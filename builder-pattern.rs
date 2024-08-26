#[derive(Debug)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color: String,
}

impl Bicycle {
    fn make(&self) -> &String {
        &self.make
    }
    fn model(&self) -> &String {
        &self.model
    }
    fn size(&self) -> i32 {
        self.size
    }
    fn color(&self) -> &String {
        &self.color
    }
}

struct BicycleBuilder {
    bicycle: Bicycle,
}

impl BicycleBuilder {
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
    fn with_make(&mut self, make: &str) {
        self.bicycle.make = make.into()
    }
    fn with_model(&mut self, model: &str) {
        self.bicycle.model = model.into()
    }
    fn with_size(&mut self, size: i32) {
        self.bicycle.size = size
    }
    fn with_color(&mut self, color: &str) {
        self.bicycle.color = color.into()
    }
    fn build(self) -> Bicycle {
        self.bicycle
    }
}

fn main() {
    let mut builder = BicycleBuilder::new();
    builder.with_make("Huffy");
    builder.with_model("Radio");
    builder.with_size(46);
    builder.with_color("black");

    let bicycle = builder.build();
    println!("{:?}", bicycle);
}