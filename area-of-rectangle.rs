fn main() {
    let rect1 = Rectangle { height: 50, width: 30};

    println!("矩形的面积是 {}", 
             area(&rect1)         
    );

    println!("使用方法计算的圆的面积是 {}",
             rect1.area()
    );

    println!("{:#?}", rect1);


    let rect01 = Rectangle { width: 30, height: 50 };
    let rect02 = Rectangle { width: 10, height: 40 };
    let rect03 = Rectangle { width: 60, height: 45 };

    println!("Can rect01 hold rect02? {}", rect01.can_hold(&rect02));
    println!("Can rect01 hold rect03? {}", rect01.can_hold(&rect03));

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
      self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
      self.width >= other.width && self.height >= other.height
  }
}
