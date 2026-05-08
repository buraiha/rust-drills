#[tokio::main]
async fn main()->std::io::Result<()> {
    println!("listening on 127.0.0.1:4001");
    day39_tcp_echo_async::run("127.0.0.1:4001").await
}
