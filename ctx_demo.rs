use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

// 定义上下文结构体
struct Context {
    connection_count: u32,
}

// 定义处理连接的异步函数
async fn handle_connection(stream: TcpStream, context: Arc<Mutex<Context>>) {
    // 增加连接计数
    {
        let mut ctx = context.lock().await;
        ctx.connection_count += 1;
        println!("新连接建立,当前连接数: {}", ctx.connection_count);
    }

    // 这里可以添加更多的连接处理逻辑
    // ...

    // 模拟一些处理时间
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // 减少连接计数
    {
        let mut ctx = context.lock().await;
        ctx.connection_count -= 1;
        println!("连接关闭,当前连接数: {}", ctx.connection_count);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建上下文并包装在 Arc 和 Mutex 中
    let context = Arc::new(Mutex::new(Context {
        connection_count: 0,
    }));

    // 绑定 TCP 监听器到地址
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器监听在 127.0.0.1:8080");

    loop {
        // 接受新的连接
        let (stream, _) = listener.accept().await?;
        
        // 克隆上下文的引用
        let context_clone = Arc::clone(&context);

        // 为每个连接创建一个新的任务
        tokio::spawn(async move {
            handle_connection(stream, context_clone).await;
        });
    }
}
