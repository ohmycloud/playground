use std::iter::Iterator;
use std::fmt::Debug;

fn use_iter<T, U>(mut iter: U)
    where U: Iterator<Item=T>,
          T: Debug
{
  while let Some(i) = iter.next() {
    println!("{:?}", i)
  }
}


/// simplefield version
fn another_use_iter<U>(mut iter: U) 
    where U: Iterator,
          U::Item: Debug
{
    while let Some(i) = iter.next() {
        println!("{:?}", i);
    }
}

fn main() {
    let v: Vec<i32> = vec![1,2,3,4,5];
    use_iter(v.iter());

    let x: Vec<i32> = vec![6,7,8,9,10];
    another_use_iter(x.iter());
}
