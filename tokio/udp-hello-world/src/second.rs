use tokio::{net::UdpSocket, sync::mpsc};
use std::{io, net::SocketAddr, sync::Arc, thread, time};
use rand;
use rand::Rng;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(sock);

    let mut buf = [0; 1024];
    loop {
		let s = r.clone();
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

		let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);
		tokio::spawn(async move {
			if let Some((bytes, addr)) = rx.recv().await {
				let random: u64 = rand::thread_rng().gen_range(0..5);
				let response = format!("rand: {}, {}", random, String::from_utf8_lossy(&bytes));

				let wait = time::Duration::from_secs(random);
				thread::sleep(wait);

				println!("{}", response);

				let len = s.send_to(response.as_bytes(), &addr).await.unwrap();
				println!("rand: {}, {:?} bytes sent", random, len);
			}
		});
	
		tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}