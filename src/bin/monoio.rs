use std::{
    thread,
    time::{Duration, Instant},
};

use monoio::{
    io::{AsyncReadRent, AsyncWriteRent},
    net::{TcpListener, TcpStream},
};
use pingo_pongo::{addr, bind_to_cpu_set, elapsed, ADDRESS, ITERATIONS};

fn main() {
    let ponger = thread::Builder::new()
        .name("ponger".to_string())
        .spawn(|| {
            bind_to_cpu_set([3usize]).unwrap();
            let mut rt = monoio::RuntimeBuilder::<monoio::IoUringDriver>::new()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let listener = TcpListener::bind(addr()).unwrap();
                println!("Listening on {}", ADDRESS);
                let (mut stream, _) = listener.accept().await.unwrap();

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
        .name("pinger".to_string())
        .spawn(|| {
            bind_to_cpu_set([4usize]).unwrap();

            let mut rt = monoio::RuntimeBuilder::<monoio::IoUringDriver>::new()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                monoio::time::sleep(Duration::from_millis(300)).await;

                let mut buf_send = vec![1u8; 4];
                let mut buf_recv = vec![0u8; 4];

                let mut stream = TcpStream::connect(addr()).await.unwrap();

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
            })
        })
        .unwrap();

    ponger.join().unwrap();
    pinger.join().unwrap();
}
