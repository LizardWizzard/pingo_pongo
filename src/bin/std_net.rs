use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::{
    thread,
    time::{Duration, Instant},
};

use pingo_pongo::{bind_to_cpu_set, elapsed, ADDRESS, ITERATIONS};

/*
const ITERATIONS: usize = 1_000_000;
elapsed ponger 14.825048553s
elapsed pinger 14.8250627s

const ITERATIONS: usize = 5_000_000;
elapsed ponger 68.659679127s
elapsed pinger 68.659682156s

*/

fn main() {
    let ponger = thread::Builder::new()
        .name("ponger".to_string())
        .spawn(|| {
            bind_to_cpu_set([3usize]).unwrap();
            let listener = TcpListener::bind(ADDRESS).unwrap();
            println!("Listening on {}", listener.local_addr().unwrap());
            let (mut stream, _) = listener.accept().unwrap();
            let buf_send = [2u8; 4];
            let mut buf_recv = [0u8; 4];
            let t0 = Instant::now();
            for _ in 0..ITERATIONS {
                stream.read(&mut buf_recv).unwrap();
                // println!("ponger read buf {:?}", buf_recv);
                stream.write(&buf_send).unwrap();
                // println!("ponger write buf {:?}", buf_send);
            }
            elapsed(t0, ITERATIONS, "elapsed ponger");
        })
        .unwrap();

    let pinger = thread::Builder::new()
        .name("pinger".to_string())
        .spawn(|| {
            bind_to_cpu_set([4usize]).unwrap();
            sleep(Duration::from_millis(100));
            let mut stream = TcpStream::connect(ADDRESS).unwrap();
            println!("Connected to {}", stream.peer_addr().unwrap());
            let buf_send = [1u8; 4];
            let mut buf_recv = [0u8; 4];
            let t0 = Instant::now();
            for _ in 0..ITERATIONS {
                stream.write(&buf_send).unwrap();
                // println!("pinger write buf {:?}", buf_send);
                stream.read(&mut buf_recv).unwrap();
                // println!("pinger read buf {:?}", buf_recv);
            }
            elapsed(t0, ITERATIONS, "elapsed pinger");
        })
        .unwrap();
    ponger.join().unwrap();
    pinger.join().unwrap();
}
