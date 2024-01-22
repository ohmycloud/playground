fn main() {
    let mut range = 1..=10;
    loop {
        match range.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    // 上面的代码等价于
    for x in 1..=10 {
        println!("{}", x);
    }

    // iterators give you a sequence of values.
    // iterator adaptors operate on an iterator, producing a new iterator with different output sequence.
    // consumers operate on an iterator, producing some final set of values.
    // consumer 是 eager 的, 一执行就会得到结果; adaptor 是 lazy 的, 并不会马上执行。
    let collect_consumer = (1..=100).collect::<Vec<_>>();
    let find_consumer = (1..=100).find(|x| *x > 42);
    let fold_consumer = (1..=100).fold(0, |sum, x| sum +x);

    let map_adaptor = (1..=100).map(|x| x + 1);
    let take_adaptor = (1..).take(5);
    let filter_adaptor = (1..=100).filter(|x| *x % 2 == 0);
    let filter_map_adaptor = (1..=5).filter_map(|x| {
        if x % 2 == 0 {
            None
        } else {
            Some(x)
        }
    });
    let float_string = vec!["a", "1.24", "3", "def", "45", "0.23"];
    let filter_map_adaptor = float_string.iter().filter_map(|x| x.parse().ok());
    assert_eq!(filter_map_adaptor.collect::<Vec<f64>>(), vec![1.24, 3.0, 45.0, 0.23]);

}