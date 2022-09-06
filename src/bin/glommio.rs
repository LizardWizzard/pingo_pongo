use std::time::{Duration, Instant};

use futures_lite::{AsyncReadExt, AsyncWriteExt};
use glommio::{
    net::{TcpListener, TcpStream},
    LocalExecutorBuilder, Placement,
};
use pingo_pongo::{elapsed, ADDRESS, ITERATIONS};

/*
const ITERATIONS: usize = 1_000_000;
elapsed ponger 25.329532184s
elapsed pinger 25.329501396s

const ITERATIONS: usize = 5_000_000;
elapsed ponger 124.571468967s
elapsed pinger 124.571442158s
*/

fn main() {
    let ponger = LocalExecutorBuilder::new(Placement::Fixed(3))
        .name("ponger")
        .spawn(move || async {
            let listener = TcpListener::bind(ADDRESS).unwrap();
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
            elapsed(t0, ITERATIONS, "elapsed ponger");
        })
        .unwrap();

    let pinger = LocalExecutorBuilder::new(Placement::Fixed(4))
        .name("pinger")
        .spawn(move || async {
            glommio::timer::sleep(Duration::from_millis(100)).await;
            let mut stream = TcpStream::connect(ADDRESS).await.unwrap();
            println!("Connected to {}", stream.peer_addr().unwrap());
            let buf_send = [1u8; 4];
            let mut buf_recv = [0u8; 4];
            let t0 = Instant::now();
            for _ in 0..ITERATIONS {
                stream.write(&buf_send).await.unwrap();
                // println!("pinger write buf {:?}", buf_send);
                stream.read(&mut buf_recv).await.unwrap();
                // if i % 1000 == 0 {
                //     println!("i {}", i)
                // }
                // println!("pinger read buf {:?}", buf_recv);
            }
            elapsed(t0, ITERATIONS, "elapsed pinger");
        })
        .unwrap();
    ponger.join().unwrap();
    pinger.join().unwrap();
}
