use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

fn main() {
    let ports = [80, 8080, 3000];
    let address = "127.0.0.1";

    let open_ports = scan(address, &ports, Duration::from_millis(500)).unwrap();
    for port in open_ports.iter() {
        println!("{}:{} OPEN", address, port)
    }
}

fn scan(address: &str, ports: &[u16], connect_timeout: Duration) -> Result<Vec<u16>, failure::Error> {
    let mut open_ports: Vec<u16> = Vec::new();
    for port in ports.iter() {
        let target = format!("{}:{}", address, port);
        let target = SocketAddr::from_str(&target)?;
        match TcpStream::connect_timeout(&target, connect_timeout) {
            Ok(_) => open_ports.push(*port),
            Err(_) => ()
        }
    }

    Ok(open_ports)
}