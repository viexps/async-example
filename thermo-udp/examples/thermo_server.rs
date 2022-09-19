use std::sync::Arc;

use tokio::{
    io,
    net::UdpSocket,
    sync::Mutex,
    time::{self, Duration},
};

struct SmartThermometer {
    temp: Arc<Mutex<Option<f32>>>,
}

impl SmartThermometer {
    fn new() -> Self {
        Self {
            temp: Arc::new(Mutex::new(None)),
        }
    }

    async fn set_temp(&mut self, temp: f32) {
        let mut t = self.temp.lock().await;
        *t = Some(temp)
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let bind_addr = "127.0.0.1:34255";
    let socket = UdpSocket::bind(bind_addr).await?;
    let mut thermo = SmartThermometer::new();

    let temp = Arc::clone(&thermo.temp);

    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_secs(1)).await;
            let t = temp.lock().await;
            let temp_str: String =
                t.map_or_else(|| "undefined".to_string(), |x| format!("{} C.", x));
            println!("current temp: {}", temp_str);
        }
    });

    let mut buf = [0u8; 4];
    let timeout = Duration::from_secs(5);
    loop {
        if let Err(e) = time::timeout(timeout, socket.recv_from(&mut buf)).await {
            println!("can't receive datagram: {e}");
            continue;
        }
        let temp = f32::from_be_bytes(buf);
        thermo.set_temp(temp).await;
    }
}
