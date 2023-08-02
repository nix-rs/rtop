use std::fs;
use std::io;
use std::collections::HashMap;
use std::io::Read;
use std::{
    thread,
    time,
};

/*----------- FUTURE ADDON -----------
 * 1. Read the /proc/<pid>/io file to tell process wise i/o for debugging
 *
 *
 *
*/

struct Processes {
    name: String,
    ppid: i32,
    state: String,
    threads: i32,
    cpu: f32,
    mem: i32,
    command: String,
    user: String,
}

fn read_file(path : &str) -> Result<String, io::Error> {
    let readfile  = fs::File::open(path)?;
    let mut buff = io::BufReader::new(readfile);
    let mut content = String::new();
    buff.read_to_string(&mut content)?;

    Ok(content)
}


/// Sorting the 'status' file
/// fields we are spitting here are:
/// * Name
/// * PPid
/// * State
/// * Threads No.
/// * VmRSS (mem)
/// * UID
pub fn status<'a>(content : &'a String) -> Result<HashMap<&'a str, &'a str>, io::Error> {
    let mut fields: HashMap<&str, &str> =  HashMap::new();

    for i in content.split("\n") {
        let v: Vec<&str> = i.split("\t").collect();
        if v[0] == "" {
            continue;
        }
        if v[0] == "Name:" || v[0] == "State:" || v[0] == "PPid:" || v[0] == "Threads:" || v[0] == "VmRSS:" || v[0] == "Uid:" {
            fields.insert(
                v[0].trim_end_matches(":"),
                v[1],
            );
        }
    }
    Ok(fields)
}

/// Here we are reading per process cpu usages
/// * CPU%
pub fn cpu_stat() {
    let total_time_before: f32 = main_cpu_ticks();
    let mut utime_before: f32 = 0.0;
    let mut utime_after: f32 = 0.0;
    let mut stime_before: f32 = 0.0;
    let mut stime_after: f32 = 0.0;

    let path = "/proc/1944/stat";
    let cont = read_file(path).unwrap();

    for (i, j) in cont.split(" ").enumerate() {
        if i == 13 {
            utime_before = j.parse::<f32>().unwrap();
        } else if i == 14 {
            stime_before = j.parse::<f32>().unwrap();
        }
    }

    // sleep for 5 sec.
    let second = time::Duration::from_secs(5);
    thread::sleep(second);

    let total_time_after: f32 = main_cpu_ticks();
    let cont = read_file(path).unwrap();

    for (i, j) in cont.split(" ").enumerate() {
        if i == 13 {
            utime_after = j.parse::<f32>().unwrap();
        } else if i == 14 {
            stime_after= j.parse::<f32>().unwrap();
        }
    }

    let user_utils: f32 = 100.0 * (utime_after - utime_before) / (total_time_after - total_time_before);
    let sys_utils: f32 = 100.0 * (stime_after - stime_before) / (total_time_after - total_time_before);
    println!("user : {}% \n sys : {}%", user_utils, sys_utils);

}

/// * command
pub fn cmdline(content: &String) {
    println!("{}", content);
}

/// Here we are reading the main 'stat' file to get the total time
/// of all CPUs cumulatively
fn main_cpu_ticks()  -> f32 {
    let path = "/proc/stat";
    let stats = read_file(path).unwrap();

    let mut ticks: f32 = 0.0;

    for i in stats.lines() {
        for j in i.split(" ") {
            //println!("{j}");
            if j.parse::<i32>().is_ok() {
                let x = j.parse::<f32>().unwrap();
                ticks += x;
            }
        }
        break;
    }
    //println!("{ticks}");

    ticks
}
