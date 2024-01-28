use std::net::TcpStream;
use std::io::Write;

pub struct ScreenClient {
    stream: TcpStream,
}

impl ScreenClient {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(
            Self { stream: TcpStream::connect("127.0.0.1:8080")? }
        )
    }

    pub fn send(&mut self, msg: (f64, f64)) {
        let msg = format!("{} {}", msg.0, msg.1);
        self.stream.write(msg.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}