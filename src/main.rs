use std::fs;
use std::io;
use std::io::Read;

mod sorting;

use crate::sorting::{
    status,
    cpu_stat,
    maps,
};

/*
 * -------------- <TODO>-----------
 *
 * 1. Read folder recursivley   [option]:crate: walkdir:
 * 2. make a function to read process with PID
 * 3. make another function for system wide reading
 *
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

    // testing file address
    let readfile  = fs::File::open("/proc/1/status")?;
    let mut buff = io::BufReader::new(readfile);
    let mut stats = String::new();
    buff.read_to_string(&mut stats)?;

    //println!("{:#?}", status(&stats)?);


    // testing file address
    let readfile1  = fs::File::open("/proc/1/stat")?;
    let mut buff1 = io::BufReader::new(readfile1);
    let mut cpu = String::new();
    buff1.read_to_string(&mut cpu)?;
    cpu_stat();

    println!("{}", maps());

    Ok(())
}


