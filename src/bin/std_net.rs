use std::{io, thread, time::{Duration, Instant}};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;

const ITERATIONS: usize = 5_000_000;
/*
const ITERATIONS: usize = 1_000_000;
elapsed ponger 14.825048553s
elapsed pinger 14.8250627s

const ITERATIONS: usize = 5_000_000;
elapsed ponger 68.659679127s
elapsed pinger 68.659682156s

*/

fn bind_to_cpu_set(cpus: impl IntoIterator<Item = usize>) -> Result<(), io::Error>{
    let mut cpuset = nix::sched::CpuSet::new();
    for cpu in cpus {
        cpuset.set(cpu)?;
    }
    let pid = nix::unistd::Pid::from_raw(0);
    nix::sched::sched_setaffinity(pid, &cpuset)?;
    Ok(())
}

fn main() {
    let ponger = thread::Builder::new().name("ponger".to_string()).spawn(|| {
        bind_to_cpu_set([3usize]).unwrap();
        let listener = TcpListener::bind("127.0.0.1:8003").unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());
        let (mut stream, _) = listener.accept().unwrap();
        let buf_send = [2u8; 4];
        let mut buf_recv = [0u8; 4];
        let t0 = Instant::now();
        for _ in 0..ITERATIONS {
            stream.read_exact(&mut buf_recv).unwrap();
            // println!("ponger read buf {:?}", buf_recv);
            stream.write_all(&buf_send).unwrap();
            // println!("ponger write buf {:?}", buf_send);
        }
        println!("elapsed ponger {:?}", t0.elapsed())
    }).unwrap();
    let pinger = thread::Builder::new().name("pinger".to_string()).spawn(|| {
        bind_to_cpu_set([4usize]).unwrap();
        sleep(Duration::from_millis(100));
        let mut stream = TcpStream::connect("127.0.0.1:8003").unwrap();
        println!("Connected to {}", stream.peer_addr().unwrap());
        let buf_send = [1u8; 4];
        let mut buf_recv = [0u8; 4];
        let t0 = Instant::now();
        for _ in 0..ITERATIONS {
            stream.write_all(&buf_send).unwrap();
            // println!("pinger write buf {:?}", buf_send);
            stream.read_exact(&mut buf_recv).unwrap();
            // println!("pinger read buf {:?}", buf_recv);
        }
        println!("elapsed pinger {:?}", t0.elapsed())
    }).unwrap();
    ponger.join().unwrap();
    pinger.join().unwrap();
}