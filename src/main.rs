#![allow(unused)]

use crossterm::event::{self, KeyEvent, KeyCode, Event};
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::process;

pub mod error;
mod sorting;
mod terminal;

use crate::terminal::get_size;

use crate::sorting::{
    Process,
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
 * $ lslogins will use to get the username wih userid
found it..

/proc/diskstats

the 6th and 10th columns are respectively read blocks and write blocks, to get the value in bytes, multiply with 512..

/sys/block/sdX/stat

the 3rd and 7th values are respectively the same as above

 */

fn main() -> io::Result<()> {

    loop {
        let processes = processes();

        for process in processes.iter() {
            let mut p = Process::new(*process);
            p.call_p();
            println!("cmd:{};\ncpu:{};\nname:{}\nmem:{}\nuser:{}\nthreads:{}\npid:{}\nppid:{}\nstate:{}",
                p.command(), p.cpu(), p.name(), p.mem(), p.user(), p.threads(), p.ppid(), p.pid(), p.state());
        }

        let mut sys = System::new();
        sys.call_s();
        println!("cpuS : {:?}; memS : {:?}; Uptime : {}; net : {:?}", sys.cpu_s(), sys.mem_s(), sys.uptime(), sys.net());
    }
    //print!("{:?}", get_size());
    Ok(())
}

// here we are reading all the running processes
fn processes() -> Vec<i32> {
    // here we are taking all processes ID
    let mut process_no: Vec<i32> = Vec::new();
    let paths = fs::read_dir("/proc").unwrap();

    for path in paths {
        let che = path
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();
        if che.parse::<i32>().is_ok() {
            process_no.push(che.parse::<i32>().unwrap())
        }
    }
    process_no
}


