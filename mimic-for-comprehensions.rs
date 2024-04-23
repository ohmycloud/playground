use itertools::Itertools;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let xs = vec![1, 1];
    let ys = vec![-2, 7];
    let points = xs.iter()
      .zip(ys.iter())
      .map(|(x, y)| Point {x: *x, y: *y})
      .collect::<Vec<Point>>();
    println!("{:?}", points);

    let a = vec![1, 2];
    let b = vec![10, 100];
    let c = vec![0.5, 0.7];
    let d = vec![3, 3];

    let kms: Vec<i32> = itertools::iproduct!(a,b,c,d);

    println!("{:?}", kms);        


}