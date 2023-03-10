use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    socket.connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;
        let mut buffer = [0u8; 1500];
        socket.recv_from(&mut buffer)?;
        println!(
            "recv: {}",
            std::str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
    }
}
