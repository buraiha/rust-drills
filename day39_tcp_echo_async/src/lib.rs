use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::{TcpListener,TcpStream};

pub async fn handle(mut s:TcpStream)->std::io::Result<()> {
    let mut buf=[0u8;1024];
    let n=s.read(&mut buf).await?;
    s.write_all(&buf[..n]).await?;
    Ok(())
}

pub async fn run(addr:&str)->std::io::Result<()> {
    let listener=TcpListener::bind(addr).await?;
    loop {
        let (s,_) = listener.accept().await?;
        tokio::spawn(async move { let _=handle(s).await; });
    }
}
