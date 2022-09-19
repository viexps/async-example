use std::mem;
use tokio::{
    io::{self, AsyncRead, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    spawn,
};

use socket_tcp::server::ServerHandle;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let addr = "127.0.0.1:7878";

    let mut s = ServerHandle::create(addr).await;
    let sh = spawn(async move { s.run().await });

    let mut stream = TcpStream::connect(addr).await?;
    let mut buf = [0u8; 128];

    let command_seq = [
        "status", "turn on", "load 100", "status", "turn off", "status", "exit",
    ];

    for cmd in command_seq {
        stream.write_all(format!("{}\r\n", cmd).as_bytes()).await?;
        let resp = read_line(&mut buf, &mut stream).await?;
        println!("response: {}", resp);
    }

    // stream.write(b"status\n")?;

    // let resp = read_line(&mut buf, &mut stream)?;
    // println!("response: {}", resp);

    // stream.write(b"exit\n")?;
    // stream.read(&mut buf)?;
    mem::drop(stream);

    _ = sh.await;
    println!("exiting");
    Ok(())
}

async fn read_line(buf: &mut [u8], r: &mut (impl AsyncRead + Unpin)) -> Result<String, io::Error> {
    let len = r.read(buf).await?;
    Ok(String::from_utf8(buf[0..len].to_vec()).unwrap())
}
