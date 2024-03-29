use std::{
    thread,
    time::{Duration, Instant},
};

use pingo_pongo::{bind_to_cpu_set, elapsed, ITERATIONS, addr};
use tokio_uring::net::{TcpListener, TcpStream};


fn main() {
    let ponger = thread::Builder::new()
        .name("ponger".to_string())
        .spawn(|| {
            bind_to_cpu_set([3usize]).unwrap();

            tokio_uring::start(async {
                let listener = TcpListener::bind(addr()).unwrap();
                println!("Listening on {}", addr());
                let (stream, _socket_addr) = listener.accept().await.unwrap();
                let mut buf_send = vec![2u8; 4];
                let mut buf_recv = vec![0u8; 4];

                let t0 = Instant::now();

                for _ in 0..ITERATIONS {
                    let r = stream.read(buf_recv).await;
                    buf_recv = r.1;
                    r.0.unwrap();

                    let r = stream.write(buf_send).await;
                    buf_send = r.1;
                    r.0.unwrap();
                }
                elapsed(t0, ITERATIONS, "elapsed ponger");
            });
        })
        .unwrap();

    let pinger = thread::Builder::new()
        .name("ponger".to_string())
        .spawn(|| {
            bind_to_cpu_set([4usize]).unwrap();

            tokio_uring::start(async {
                tokio::time::sleep(Duration::from_millis(100)).await;

                let mut buf_send = vec![1u8; 4];
                let mut buf_recv = vec![0u8; 4];

                let stream = TcpStream::connect(addr())
                    .await
                    .unwrap();

                let t0 = Instant::now();

                for _ in 0..ITERATIONS {
                    let r = stream.write(buf_send).await;
                    buf_send = r.1;
                    r.0.unwrap();

                    let r = stream.read(buf_recv).await;
                    buf_recv = r.1;
                    r.0.unwrap();
                }
                elapsed(t0, ITERATIONS, "elapsed pinger");
            });
        })
        .unwrap();

    ponger.join().unwrap();
    pinger.join().unwrap();
}
