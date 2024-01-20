fn main() {
    let x = "world";
    let width = 7;
    println!("|Hello {x:width$}!|");

    let items = vec![0; 3];
    println!("{items:#?}");

    let items = ["these", "words", "are", "ddifferent", "sizes"];
    let column1 = "item";
    let column2 = "iter";

    println!("|{column1:49}| {column2:-20}|");
    println!("|-----------------------------------------------------------------------|");
    for (i, item) in items.iter().enumerate() {
        println!("|{item:>49}: {i:-20}|");
    }
}
