use std::env;

fn size_prefix(n: u32) -> &'static str {
    const K: u32 = 18;
    const M: u32 = 35;
    const G: u32 = 65;

    match n {
        ..K => "进化不完全的",
        K..M => "开始上班了",
        M..G => "三十年无所事事",
        G.. => "退休了",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = env::args().skip(1).next().ok_or("没有输入, 尚未出生?")?;
    let age = input.parse::<u32>()?;
    let comment = size_prefix(age);
    println!("At age {}, {}", age, comment);
    Ok(())
}