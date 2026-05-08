use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};

pub fn handle(mut s:TcpStream)->std::io::Result<()> {
    let mut buf=[0u8;1024];
    let n=s.read(&mut buf)?;
    s.write_all(&buf[..n])?;
    Ok(())
}

pub fn run(addr:&str)->std::io::Result<()> {
    let listener=TcpListener::bind(addr)?;
    for stream in listener.incoming(){ handle(stream?)?; }
    Ok(())
}
