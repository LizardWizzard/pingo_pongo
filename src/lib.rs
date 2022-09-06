use std::{io, net::SocketAddr, time};

pub const ITERATIONS: u32 = 2_000_000;
pub const ADDRESS: &str = "127.0.0.1:8001";

pub fn addr() -> SocketAddr {
    ADDRESS.parse().unwrap()
}

pub fn bind_to_cpu_set(cpus: impl IntoIterator<Item = usize>) -> Result<(), io::Error> {
    let mut cpuset = nix::sched::CpuSet::new();
    for cpu in cpus {
        cpuset.set(cpu)?;
    }
    let pid = nix::unistd::Pid::from_raw(0);
    nix::sched::sched_setaffinity(pid, &cpuset)?;
    Ok(())
}

pub fn elapsed(t: time::Instant, iterations: u32, extra: &str) {
    let elapsed = t.elapsed();
    println!(
        "{} {:?} per iteration {:?} per second {:?}",
        extra,
        elapsed,
        elapsed / iterations,
        iterations as f64 / elapsed.as_secs_f64()
    )
}
