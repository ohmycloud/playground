fn main() {
    // unwrap_or_else on Option,
    // returns the contained Some value or computes it from a closure.
    let k = 10;
    println!("{}", Some(4).unwrap_or_else(|| 2 * k));
    println!("{}", None.unwrap_or_else(|| 2 * k));
  
    // unwrap_or_else on Result, 
    // returns the contained Ok value or computes it from a closure.
    fn count(x: &str) -> usize { x.len() }
    println!("{}", Ok(2).unwrap_or_else(count));
    println!("{}", Err("foo").unwrap_or_else(count));
  }  