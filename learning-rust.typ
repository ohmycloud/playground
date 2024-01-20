= Rust 设计模式

#let chapter(title, subtitle, heading) = [
  #grid(
    columns: (auto, auto, auto),
    rows: (auto, auto),
    row-gutter: 10pt,
    column-gutter: (10pt, 25pt),
    text(gray, weight: "bold", size: 24pt)[#title],
    [#line(length: 1.8cm, stroke: 3pt + red, angle: 90deg)],
    align(center)[#text(red, weight: "bold",baseline: 30pt, size: 24pt)[#heading]],
    align(right)[#text(red, weight: "bold", baseline: -30pt, size: 24pt,)[#subtitle]],
  )
]

== #chapter("强制解引用:", "Deref", "&str VS &String")

#v(1em)

作为函数参数时, 是使用 `&str` 还是 `&String` 呢?

#link("https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html")[Rust 设计模式] 中倾向于使用 `&str`, 因为这样函数可以接收的参数类型更多, 使用更灵活:

```rust
/// 使用 &str VS 使用 &String 
/// 使用 &str 作为参数, 函数可以接收的参数类型更多
/// From &T to &U when T: Deref<Target=U>
fn char_count(word: &str) -> usize {
    word.chars().count()
}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    
    // String 实现了 Deref, Target=str
    // &String 被强制解引用为 &str, 以匹配函数签名
    println!("{}: {}", ferris,  char_count(&ferris));
    println!("{}: {}", curious, char_count(&curious));

    println!("{}: {}", ferris, char_count("Ferris")); 

    // Box 实现了 Deref, Target=str
    let boxed_string = Box::new("hello world");
    println!("{}: {}", ferris, char_count(&boxed_string)); 

    // Box 实现了 Deref, Target=String
    // String 实现了 Deref, target=str
    // 所以这里经过了两次强制解引用
    let boxed_string = Box::new("hello world".to_string());
    println!("{}: {}", ferris, char_count(&boxed_string)); 

    // Rc 实现了 Deref, Target=String
    // String 实现了 Deref, target=str
    // 所以这里也经过了两次强制解引用
    let counted_string = std::rc::Rc::new("hello world".to_string());
    println!("{}: {}", ferris, char_count(&counted_string)); 

    // Arc 实现了 Deref, Target=str
    let atomically_counted_string = std::sync::Arc::new("hello world");
    println!("{}: {}", ferris, char_count(&atomically_counted_string)); 
}
```