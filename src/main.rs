#![feature(let_chains)]

use std::collections::HashMap;

use procfs::{
    process::{self, Process, FDTarget, Stat}, ProcResult,
};

struct Connection {
    local_address: String,
    remote_address: String,
    state: String,
    inode: u64,
    pid: u32,
    comm: String,
}

fn get_tcp() -> Result<(), Box<dyn std::error::Error>> {

    let all_procs = procfs::process::all_processes().unwrap();

    let mut map: HashMap<u64, Stat> = HashMap::new();

    for process_result in all_procs {
        if let Ok(process) = process_result 
        && let (Ok(stat), Ok(fdt)) = (process.stat(), process.fd()) {
            for fd_result in fdt {
                if let Ok(fd) = fd_result
                && let FDTarget::Socket(inode) = fd.target {
                    map.insert(inode, stat.clone());
                }
            }
        }
    }

    let tcp = procfs::net::tcp().unwrap();

    println!(
        "{:<26} {:<26} {:<15} {:<8} {}",
        "Local address", "Remote address", "State", "Inode", "PID/Program name"
    );

    for entry in tcp.iter() {
        let local_address = format!("{}", entry.local_address);
        let remote_address = format!("{}", entry.remote_address);
        let state = format!("{:?}", entry.state);
        if let Some(stat) = map.get(&entry.inode) {
            println!(
                "{:<26} {:<26} {:<15} {:<12} {}/{}",
                local_address, remote_address, state, entry.inode, stat.pid, stat.comm
            );
        }
    }

    println!();

    Ok(())
}

fn main() {
    loop {
        get_tcp();
    }
}
