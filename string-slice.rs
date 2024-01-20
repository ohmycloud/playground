fn main() {
    let s = first_word(&String::from("Hello world!"));
    println!("{}", s);

    let str = String::from("hello world");

    let hello = &str[0..5];
    let world = &str[6..11];
    println!("{} {}", hello, world);

   let star = String::from("Rakudo Star"); 
   let slice = slice_word(&star);
   println!("{}", slice);
}

fn slice_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i]
      }
    }
    &s[..]
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
           return i; // If we find a space, we return the position.
       }
    }
    s.len() // Otherwise, we return the length of the string by using s.len()
}
