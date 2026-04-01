use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;

// 定义消息类型
#[derive(Clone)]
enum Message {
    ClientMessage { id: usize, content: String },
    NewClient { id: usize },
    ClientDisconnected { id: usize },
}

// 定义服务器上下文
struct ServerContext {
    clients: HashMap<usize, mpsc::UnboundedSender<String>>,
    next_id: usize,
}

impl ServerContext {
    fn new() -> Self {
        ServerContext {
            clients: HashMap::new(),
            next_id: 0,
        }
    }

    fn add_client(&mut self, tx: mpsc::UnboundedSender<String>) -> usize {
        let id = self.next_id;
        self.clients.insert(id, tx);
        self.next_id += 1;
        id
    }

    fn remove_client(&mut self, id: usize) {
        self.clients.remove(&id);
    }

    fn broadcast(&mut self, sender_id: usize, message: &str) {
        for (&id, tx) in self.clients.iter() {
            if id != sender_id {
                let _ = tx.send(message.to_string());
            }
        }
    }
}

async fn handle_client(
    mut stream: TcpStream,
    id: usize,
    ctx: Arc<Mutex<ServerContext>>,
    tx: mpsc::UnboundedSender<Message>,
    mut rx: mpsc::UnboundedReceiver<String>,
) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = stream.split();
    let mut lines = tokio::io::BufReader::new(reader).lines();

    // 发送欢迎消息
    writer.write_all(b"Welcome to the chat server!\n").await?;

    loop {
        tokio::select! {
            result = lines.next_line() => match result {
                Ok(Some(message)) => {
                    let message = message.trim().to_string();
                    if !message.is_empty() {
                        tx.send(Message::ClientMessage { id, content: message })?;
                    }
                }
                Ok(None) => break,
                Err(_) => break,
            },
            Some(message) = rx.recv() => {
                writer.write_all(message.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        }
    }

    tx.send(Message::ClientDisconnected { id })?;
    Ok(())
}

async fn run_server(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    let ctx = Arc::new(Mutex::new(ServerContext::new()));
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    loop {
        tokio::select! {
            Ok((stream, addr)) = listener.accept() => {
                println!("New client connected: {}", addr);
                let tx_clone = tx.clone();
                let ctx_clone = Arc::clone(&ctx);

                let (client_tx, client_rx) = mpsc::unbounded_channel();
                let id = {
                    let mut ctx = ctx.lock().await;
                    ctx.add_client(client_tx)
                };

                tx.send(Message::NewClient { id })?;

                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, id, ctx_clone, tx_clone, client_rx).await {
                        eprintln!("Error handling client {}: {}", id, e);
                    }
                });
            }
            Some(message) = rx.recv() => {
                match message {
                    Message::ClientMessage { id, content } => {
                        let mut ctx = ctx.lock().await;
                        let broadcast_msg = format!("Client {}: {}", id, content);
                        println!("{}", broadcast_msg);
                        ctx.broadcast(id, &broadcast_msg);
                    }
                    Message::NewClient { id } => {
                        let mut ctx = ctx.lock().await;
                        let msg = format!("Client {} has joined the chat", id);
                        println!("{}", msg);
                        ctx.broadcast(id, &msg);
                    }
                    Message::ClientDisconnected { id } => {
                        let mut ctx = ctx.lock().await;
                        ctx.remove_client(id);
                        let msg = format!("Client {} has left the chat", id);
                        println!("{}", msg);
                        ctx.broadcast(id, &msg);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");
    run_server(listener).await
}
