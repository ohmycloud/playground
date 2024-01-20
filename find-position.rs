fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    data
    .iter()
    .enumerate()
    .find(|x| (*x).1 == &v)
    .map(|x| x.0)
}

fn main() {
    let data = vec![10, 42, 9, 8];
    let v = 42;
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
}