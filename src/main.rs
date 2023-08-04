#![allow(unused)]

use std::fs;
use std::io;
use std::io::Read;

mod sorting;

use crate::sorting::{
    Processes,
    System,
};

/*
 * -------------- <TODO>-----------
 *
 * 1. Read folder recursivley   [option]:crate: walkdir:
 * 2. make a function to read process with PID
 * 3. make another function for system wide reading
 *
 *  we read /sys for some other info
 *

found it..

/proc/diskstats

the 6th and 10th columns are respectively read blocks and write blocks, to get the value in bytes, multiply with 512..

/sys/block/sdX/stat

the 3rd and 7th values are respectively the same as above

 */

const _LIST: [&str; 8] = ["maps", "numa_maps", "oom_score_adj", "smaps", "stat", "status", "syscall", "task/"];

fn main() -> io::Result<()> {

    // here we are taking all processes ID
    let mut process_no: Vec<i32> = Vec::new();
    let paths = fs::read_dir("/proc")?;

    for path in paths {
        let che = path?
            .file_name()
            .into_string()
            .unwrap();

        if che.parse::<i32>().is_ok() {
            process_no.push(che.parse::<i32>().unwrap())
        }
    }

    //let mut process = Processes::new(2201);
    //process.call();
    //println!("name: '{}'; mem: {}; threads: {}; state: {}; cpu: {}; command: {}; user: {}; pid: {}",
    //    process.name(), process.mem(), process.threads(), process.state(), process.cpu(), process.command(), process.user(), process.pid());

    let mut sys = System::new();
    println!("{:?}", sys.mem_s());
    Ok(())
}


