use std::sync::OnceLock;
use std::cell::OnceCell;
use std::sync::{Once, ONCE_INIT};

static WINNER: OnceLock<&str> = OnceLock::new();

// `OnceCell<Vec<i32>>` cannot be shared between threads safely
// static CACHE: OnceCell<Vec<i32>> = OnceCell::new();

static mut CACHE_ONCE: Option<Vec<i32>> = None;
static ONCE: Once = ONCE_INIT;

fn once_cell() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let init_value: &String = cell.get_or_init(|| "Rakudo Star".to_string());
    assert_eq!(init_value, "Rakudo Star");
    assert!(cell.get().is_some());
    println!("{:?}", cell.get());
}

/// OnceCell 是一种单线程数据结构, 用于数据懒加载。因此它不能用在多线程场景下, 会导致数据竞争问题。
/// 如果要用于多线程环境, 请使用 std::sync::Once.
/*
fn unsafety_once_cell() {
    fn get_data() -> &'static Vec<i32> {
        CACHE.get_or_init(|| {
            let data = vec![1,2,3,4,5];
            println!("Initializing cache");
            data
        })
    }

    let data = get_data();
    println!("Data: {:?}", data);

    let data = get_data();
    println!("Data: {:?}", data);
}
*/

fn safety_once() {
    fn get_data() -> &'static Vec<i32> {
        unsafe {
            ONCE.call_once(|| {
                let data = vec![1,2,3,4,5];
                CACHE_ONCE = Some(data);
                println!("Initializing cache once");
            });
            CACHE_ONCE.as_ref().unwrap()
        }
    }

    let data = get_data();
    println!("Data: {:?}", data);

    let data = get_data();
    println!("Data: {:?}", data);
}

fn once_lock() {
    let cell = std::thread::scope(|s| {
        s.spawn(|| WINNER.set("thread"));
        std::thread::yield_now();
        WINNER.get_or_init(|| "Rakudo")
    });

    println!("{cell:?} wins!");
}

fn main() {
    // once_cell();
    // unsafety_once_cell();
    safety_once();
}
