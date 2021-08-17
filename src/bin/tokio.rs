use std::{io, thread, time::{Duration, Instant}};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};


const ITERATIONS: usize = 5_000_000;

/*
const ITERATIONS: usize = 1_000_000;
elapsed ponger 17.690839296s
elapsed pinger 17.690856714s

const ITERATIONS: usize = 5_000_000;
elapsed ponger 95.130277045s
elapsed pinger 95.130297008s
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
    let ponger = thread::spawn(|| {
        bind_to_cpu_set([3usize]).unwrap();
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async {
            let listener = TcpListener::bind("127.0.0.1:8002").await.unwrap();
            println!("Listening on {}", listener.local_addr().unwrap());
            let (mut stream, _) = listener.accept().await.unwrap();
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
    });
    let pinger = thread::spawn(|| {
        bind_to_cpu_set([4usize]).unwrap();
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let mut stream = TcpStream::connect("127.0.0.1:8002").await.unwrap();
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
    });
    ponger.join().unwrap();
    pinger.join().unwrap();
}