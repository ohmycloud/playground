// 定义不同的状态
struct Empty;
struct Awaiting;
struct Ready;

// 定义一个 Connection 结构体，使用泛型参数 S 来表示状态
struct Connection<S> {
    address: String,
    state: std::marker::PhantomData<S>,
}

// 为不同状态的 Connection 实现不同的方法
impl Connection<Empty> {
    fn new(addr: impl Into<String>) -> Connection<Empty> {
        Connection {
            address: addr.into(),
            state: std::marker::PhantomData,
        }
    }

    fn connect(self) -> Connection<Awaiting> {
        println!("Connecting to {}...", self.address);
        Connection {
            address: self.address,
            state: std::marker::PhantomData,
        }
    }
}

impl Connection<Awaiting> {
    fn establish(self) -> Connection<Ready> {
        println!("Connection established!");
        Connection {
            address: self.address,
            state: std::marker::PhantomData,
        }
    }
}

impl Connection<Ready> {
    fn send_data(&self, data: &str) {
        println!("Sending data: {}", data);
    }

    fn close(self) -> Connection<Empty> {
        println!("Closing connection...");
        Connection {
            address: self.address,
            state: std::marker::PhantomData,
        }
    }
}

fn main() {
    // 使用示例
    let conn = Connection::new("localhost:8080")  // 类型是 Connection<Empty>
        .connect()                                // 类型变成 Connection<Awaiting>
        .establish();                            // 类型变成 Connection<Ready>
    
    conn.send_data("Hello, World!");            // 只有在 Ready 状态才能发送数据
    let _ = conn.close();                       // 关闭连接，返回 Empty 状态
}
