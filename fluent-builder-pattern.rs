macro_rules! with_str {
    ($name:ident, $func:ident) => {
        pub fn $func(self, $name: &str) -> Self {
            Self {
                bicycle: Bicycle {
                    $name: $name.into(),
                    ..self.bicycle
                },
            }
        }
    };
}

macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        pub fn $func(self, $name: $type) -> Self {
            Self {
                bicycle: Bicycle {
                    $name,
                    ..self.bicycle
                },
            }
        }
    };
}

macro_rules! accessor {
    ($name:ident, &$type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> $type {
            self.$name
        }
    }
}

#[derive(Debug, Default)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    colour: String
}

struct BicycleBuilder {
    bicycle: Bicycle,
}

pub trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}

pub trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

impl Bicycle {
    accessor!(make, &String);
    accessor!(model, &String);
    accessor!(size, i32);
    accessor!(colour, &String);
}

impl BicycleBuilder {
    with_str!(make, with_make);
    with_str!(model, with_model);
    with!(size, with_size, i32);
    with_str!(colour, with_colour);
}

impl Builder<Bicycle> for BicycleBuilder {
    fn new() -> Self {
        Self {
            bicycle: Bicycle::default()
        }
    }
    fn build(self) -> Bicycle {
        self.bicycle
    }
}

impl Buildable<Bicycle, BicycleBuilder> for Bicycle {
    fn builder() -> BicycleBuilder {
        BicycleBuilder::new()
    }
}


fn main() {
    let bicycle = Bicycle::builder()
        .with_make("上海")
        .with_model("鳯凰")
        .with_size(42)
        .with_colour("天蓝")
        .build();
    println!("我的自行车：{:?}", bicycle);
}
