fn main() {
    let some_int = Some(7);
    // convert Option into Vec
    let int_vec = some_int.into_iter().map(|x| x).collect::<Vec<i32>>();
    for i in int_vec {
        println!("{:#?}", i);
    }
}