use itertools::Itertools;

// cargo add itertools
// input 表示一组结果, 有成功执行的结果(用 `Ok<T>` 表示), 也有失败的错误信息(用 `Err<T>` 表示).
// 如果想对成功的结果进一步做 filter/map, 那么标准库就无法帮忙了, 就需要用 itertools 里的 filter_map_ok.
// `filter_map_ok` 原样返回错误信息, 只对 `Ok` 值进行 filter/map 操作,
// 所以下面的代码中, 变量 it 处理包含 Ok 值之外, 还会包含 `Err("bad happened")`. 
fn main() {
    let err_str = "bad happened";
    let input = vec![Ok(21), Err(err_str), Ok(7)];

    let it = input
        .into_iter()
        .filter_map_ok(|i| if i > 10 { Some(i * 2) } else { None });

    // [Ok(42), Err("bad happened")]    
    println!("{:?}", it.collect::<Vec<_>>());
}
