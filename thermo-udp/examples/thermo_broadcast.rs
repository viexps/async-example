use tokio::io;
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

struct TempGenerator {
    base_temp: f32,
    seq: i32,
}

impl TempGenerator {
    fn new(base_temp: f32) -> Self {
        Self { base_temp, seq: 0 }
    }

    fn generate(&mut self) -> f32 {
        let diff = (self.seq % 20 - 10) as f32 / 10.0;
        self.seq += 1;
        return self.base_temp + diff;
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    println!("Broadcaster");
    let socket = UdpSocket::bind("127.0.0.1:34254").await?;

    let dest = "127.0.0.1:34255";
    let mut gen = TempGenerator::new(20.0);

    loop {
        let temp = gen.generate();
        println!("temp: {}", temp);
        socket.send_to(&temp.to_be_bytes(), dest).await.ok();
        sleep(Duration::from_secs(1)).await;
    }
}
