use std::time::{Duration, Instant};

use futures_lite::{AsyncReadExt, AsyncWriteExt};
use glommio::{
    net::{TcpListener, TcpStream},
    LocalExecutorBuilder,
};

const ITERATIONS: usize = 5_000_000;
/* 
const ITERATIONS: usize = 1_000_000;
elapsed ponger 25.329532184s
elapsed pinger 25.329501396s

const ITERATIONS: usize = 5_000_000;
elapsed ponger 124.571468967s
elapsed pinger 124.571442158s
*/

fn main() {
    let ponger = LocalExecutorBuilder::new()
        .pin_to_cpu(2)
        .name("ponger")
        .spawn(move || async {
            let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
            println!("Listening on {}", listener.local_addr().unwrap());
            let mut stream = listener.accept().await.unwrap();
            let buf_send = [2u8; 4];
            let mut buf_recv = [0u8; 4];
            let t0 = Instant::now();
            for _ in 0..ITERATIONS {
                stream.read_exact(&mut buf_recv).await.unwrap();
                // println!("ponger read buf {:?}", buf_recv);
                stream.write_all(&buf_send).await.unwrap();
                // println!("ponger write buf {:?}", buf_send);
            }
            println!("elapsed ponger {:?}", t0.elapsed())
        })
        .unwrap();
    let pinger = LocalExecutorBuilder::new()
        .pin_to_cpu(3)
        .name("pinger")
        .spawn(move || async {
            glommio::timer::sleep(Duration::from_millis(100)).await;
            let mut stream = TcpStream::connect("127.0.0.1:8001").await.unwrap();
            println!("Connected to {}", stream.peer_addr().unwrap());
            let buf_send = [1u8; 4];
            let mut buf_recv = [0u8; 4];
            let t0 = Instant::now();
            for _ in 0..ITERATIONS {
                stream.write_all(&buf_send).await.unwrap();
                // println!("pinger write buf {:?}", buf_send);
                stream.read_exact(&mut buf_recv).await.unwrap();
                // println!("pinger read buf {:?}", buf_recv);
            }
            println!("elapsed pinger {:?}", t0.elapsed())
        })
        .unwrap();
    ponger.join().unwrap();
    pinger.join().unwrap();
}
