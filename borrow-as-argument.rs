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
