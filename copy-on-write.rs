use std::borrow::Cow;

// accept a Cow type
fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
}

// 如果字符串包含 rakulang, 则追加 Rocks 并返回
// 否则返回引用的值
// by returning a Cow the function says "I'm returning an existing string or a new string"
// You will usually see Cow as function return types.
// but it could basically be called OwnedOrBorrowed
fn rocks(input: &str) -> Cow<str> {
    if input.contains("rakulang") {
        let res = format!("{} Rocks!", input);
        Cow::Owned(res)
    } else {
        Cow::Borrowed(input)
    }
}

/// Imagine you have a function fn to_lowercase(&str) -> String,
/// but you know that in most cases, input will already be lowercased.
/// You could change it to (or make a new wrapper function) return fn to_lowercase<'a>(&'a str) -> Cow<'a, str> and now,
/// if string doesn't require any changes, it will borrow original string, avoiding allocation of String
fn to_lowercase(input: &str) -> Cow<str> {
    if input.chars().all(|c| c.is_lowercase()) {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(input.to_lowercase())
    }
}


fn main() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("{:?}", input);

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, -1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("{:?}", input);

    let slice = "rakulang";
    let language = rocks(slice);
    println!("{:?}", language);

    let input = "rakulang ROCKS";
    let output = to_lowercase(input);
    println!("{:?}", output);
}
