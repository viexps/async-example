use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::SmartSocket;

pub struct ServerHandle {
    listener: TcpListener,
    state: State,
}

struct State {
    socket: SmartSocket,
}

impl ServerHandle {
    pub async fn create(addr: impl ToSocketAddrs) -> Self {
        let listener = TcpListener::bind(addr).await.unwrap();
        let state = State {
            socket: SmartSocket::new(),
        };
        Self { listener, state }
    }

    pub async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            println!("new client: {:?}", addr);
            ServerHandle::handle_client(stream, &mut self.state)
                .await
                .ok();
        }
    }

    async fn handle_client(mut stream: TcpStream, state: &mut State) -> Result<(), io::Error> {
        let mut buf = [0u8; 128];
        loop {
            let read = stream.read(&mut buf).await?;
            if read == 0 {
                continue;
            }
            let bytes = &buf[..read];

            if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                let s = s
                    .strip_suffix("\r\n")
                    .or_else(|| s.strip_suffix('\n'))
                    .unwrap_or(&s);

                println!("received string: {}", s);
                let parts = s.split('\n');

                for msg in parts {
                    if msg == "exit" {
                        return Ok(());
                    }

                    match process_message(state, msg) {
                        Ok(resp) => {
                            stream.write_all(format!("{}\n", resp).as_bytes()).await?;
                        }
                        Err(error_str) => {
                            stream
                                .write_all(format!("{}\r\n", error_str).as_bytes())
                                .await?;
                        }
                    }
                }
            }
        }
    }
}

fn process_message(state: &mut State, msg: impl AsRef<str>) -> Result<String, &'static str> {
    let msg = msg.as_ref();
    if msg == "status" {
        Ok(state.socket.to_string())
    } else if msg == "turn on" {
        state.socket.turn_on();
        Ok("+OK".to_string())
    } else if msg == "turn off" {
        state.socket.turn_off();
        Ok("+OK".to_string())
    } else if msg.starts_with("load ") {
        let msg = msg
            .strip_prefix("load ")
            .ok_or("error while parsing load")?;
        let load: u32 = msg.parse().map_err(|_| "error while parsing load")?;
        state.socket.set_load(load);
        Ok("+OK".to_string())
    } else {
        Err("-unknown command")
    }
}
