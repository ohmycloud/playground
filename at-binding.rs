fn main() {
    new_bindings();
}

#[derive(Debug)]
struct Matrix {
    data: Vec<f64>,
    row_len: usize,
}

fn new_bindings() {
    let matrix @ Matrix { row_len, ..} = Matrix { 
        data: vec![0f64;5],
        row_len: 42, 
    };
    println!("{:?}", matrix);
    println!("{:?}", row_len);
    
}