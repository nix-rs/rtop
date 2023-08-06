use std::error::Error;
use std::fs;
use std::fs::read_link;
use std::io::{self, ErrorKind};
use std::collections::HashMap;
use std::io::Read;
use std::{
    thread,
    time,
};

use crate::error::CustomError;

/*----------- FUTURE ADDON -----------
 * 1. Read the /proc/<pid>/io file to tell process wise i/o for debugging
 *
 *
 *
*/

enum State {
    Running,
    Sleeping,

}

fn read_file(path : &String) -> Result<String, io::Error> {
    let readfile  = fs::File::open(path)?;
    let mut buff = io::BufReader::new(readfile);
    let mut content = String::new();
    buff.read_to_string(&mut content)?;

    Ok(content)
}

/// Here we are reading the main 'stat' file to get the total time
/// of all CPUs cumulatively
fn main_cpu_ticks()  -> f32 {
    let path = "/proc/stat".to_string();
    let stats = read_file(&path).unwrap();

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

#[derive(Debug)]
pub struct Process {
    name: String,
    ppid: i32,
    state: String,
    threads: i32,
    cpu: f32,
    mem: i32,
    command: String,
    user: String,
    pid: i32,
}

impl Process {
    pub fn new(id: i32) -> Self {
        Process {
            name: "".to_string(),
            ppid: 0,
            state: "".to_string(),
            threads: 0,
            cpu: 0.0,
            mem: 0,
            command: "".to_string(),
            user: "".to_string(),
            pid: id
        }
    }

    pub fn call_p(&mut self) {
        &self.status();
        &self.cpu_stat();
        &self.cmdline();
    }
    /// Sorting the 'status' file
    /// fields we are spitting here are:
    /// * Name
    /// * PPid
    /// * State
    /// * Threads No.
    /// * VmRSS (mem)
    /// * UID
    fn status(&mut self) {
        let path = format!("/proc/{}/status", self.pid);
        println!("E---->{}",&path);
        let content = match read_file(&path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => "".to_string(),
                _ => panic!("EEEEEEEEEEEE out"),
            },
        };
        let mut fields: HashMap<&str, &str> = HashMap::new();

        for i in content.split("\n") {
            let v: Vec<&str> = i.split("\t").collect();
            if v[0] == "" {
                continue;
            }
            if v[0] == "Name:" || v[0] == "State:" || v[0] == "PPid:" || v[0] == "Threads:" || v[0] == "VmRSS:" || v[0] == "Uid:" {
                if v[0] == "VmRSS:" {
                    fields.insert(
                        v[0].trim_end_matches(":"),
                        v[1].trim_start().trim_end_matches("kB").trim_end(),
                    );
                } else {
                    fields.insert(
                        v[0].trim_end_matches(":"),
                        v[1]
                    );
                }
            }
        }

        self.name = (*fields.get("Name").unwrap_or(&"n")).to_string();
        self.state = (*fields.get("State").unwrap_or(&"s")).to_string();
        self.ppid = (*fields.get("PPid").unwrap_or(&"0")).parse::<i32>().unwrap();
        self.threads = (*fields.get("Threads").unwrap_or(&"0")).parse::<i32>().unwrap();
        self.mem = (*fields.get("VmRSS").unwrap_or(&"0")).parse::<i32>().unwrap();
        self.user = (*fields.get("Uid").unwrap_or(&"0")).to_string();
    }

    /// Here we are reading per process cpu usages
    /// * CPU%
    fn cpu_stat(&mut self) {
        let total_time_before: f32 = main_cpu_ticks();
        let mut utime_before: f32 = 0.0;
        let mut utime_after: f32 = 0.0;
        let mut stime_before: f32 = 0.0;
        let mut stime_after: f32 = 0.0;

        let path = format!("/proc/{}/stat", self.pid);
        let cont = match read_file(&path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => "".to_string(),
                _ => panic!("EEEEEEEEEEEE out"),
            },
        };

        for (i, j) in cont.split(" ").enumerate() {
            if i == 13 {
                utime_before = j.parse::<f32>().unwrap();
            } else if i == 14 {
                stime_before = j.parse::<f32>().unwrap();
            }
        }

        // sleep for 5 sec.
        let second = time::Duration::from_secs(1);
        thread::sleep(second);

        let total_time_after: f32 = main_cpu_ticks();
        let cont = match read_file(&path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => "".to_string(),
                _ => panic!("EEEEEEEEEEEE out"),
            },
        };

        for (i, j) in cont.split(" ").enumerate() {
            if i == 13 {
                utime_after = j.parse::<f32>().unwrap();
            } else if i == 14 {
                stime_after= j.parse::<f32>().unwrap();
            }
        }

        let user_utils: f32 = 100.0 * (utime_after - utime_before) / (total_time_after - total_time_before);
        //let sys_utils: f32 = 100.0 * (stime_after - stime_before) / (total_time_after - total_time_before);
        if cont != "".to_string() {
            self.cpu = user_utils;
        } else {
            self.cpu = 0.0;
        }
    }

    /// * command
    fn cmdline(&mut self) {
        let path = format!("/proc/{}/cmdline", self.pid);
        let content = match read_file(&path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => "".to_string(),
                _ => panic!("EEEEEEEEEEEE out"),
            },
        };

        if content != "".to_string() {
            self.command = content;
        } else {
            self.command = "c/m/d".to_string();
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn ppid(&self) -> &i32 {
        &self.ppid
    }

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn threads(&self) -> &i32 {
        &self.threads
    }

    pub fn mem(&self) -> &i32 {
        &self.mem
    }

    pub fn user(&self) -> &String {
        &self.user
    }

    pub fn pid(&self) -> &i32 {
        &self.pid
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn cpu(&self) -> &f32 {
        &self.cpu
    }
}




pub struct System {
    process_nos: i32,
    cpu_s: Vec<f32>,
    io_s: Vec<i32>,
    net: Vec<String>,
    uptime: String,
    //version: String,
    mem_s: Vec<i32>,
}
// ------<TODO>-------
// replace  vec with hard coded array
impl System {
    pub fn new() -> Self {
        Self {
            process_nos: 0,
            cpu_s: Vec::new(),
            io_s: Vec::new(),
            net: Vec::<String>::new(),
            uptime: "".to_string(),
            //version: "".to_string(),
            mem_s: Vec::new(),
        }
    }

    fn uptime_c(&mut self) {
        //let pathV = "/proc/version".to_string();
        let path = "/proc/uptime".to_string();
        //let contentV = read_file(&pathV).unwrap();
        let content = read_file(&path).unwrap();
        let v: Vec<&str> = content.split(" ").collect();
        println!("V: '{}'", v[0]);
        let uptime = v[0].parse::<f32>().unwrap();

        let sec =&uptime % 60.0;
        let min = ((&uptime - sec) / 60.0) % 60.0;
        let hr = (((&uptime - sec) / 60.0) - min ) / 60.0;

        self.uptime = format!("{}:{}:{}", hr, min, sec.round());
    }

    // format -> ["dev_name", "recv bytes", "send_bytes"]
    fn inet(&mut self) {
        let path = "/proc/net/dev".to_string();
        let content = read_file(&path).unwrap();

        let mut v: Vec<String>= Vec::new();

        for (i, j) in content.split("\n").enumerate() {
            if i == 0 || i == 1 {
                continue
            }
            for (x, y) in j.split_whitespace().enumerate() {
                if x == 0 || x == 1 || x == 9 {
                    v.push(y.to_string());
                }
            }
        }

        self.net = v;
    }

    // format for output ----> [cpu0, cpu1, cpu2, cpu3, cpu4, cpu5, cpu6, cpu7, cpu]
    // in '%'
    fn cpu(&mut self) {
        let path = "/proc/stat".to_string();
        let content = read_file(&path).unwrap();

        let mut set_before: HashMap<&str, [i32;2]> = HashMap::new();
        'mai: for i in content.lines() {
            let mut total = 0;
            let mut idle = 0;
            let mut name = "";
            for (j, k) in i.split_whitespace().enumerate() {
                if j == 0 && ( k == "intr"
                    || k == "page"
                    || k == "swap"
                    || k == "disk_io"
                    || k == "ctxt"
                    || k == "btime"
                    || k == "processes"
                    || k == "procs_running"
                    || k == "procs_blocked"
                    || k == "softirq" ) {
                    break 'mai;
                }
                if j == 0 {
                    name = k;
                } else {
                    total += k.parse::<i32>().unwrap();
                }
                if j % 10 == 4 {
                    idle = k.parse::<i32>().unwrap();
                }
            }
            set_before.insert(
                name,
                [total, idle]);
        }

         // sleep for 5 sec.
        let second = time::Duration::from_secs(0);
        thread::sleep(second);

        let content1 = read_file(&path).unwrap();

        let mut set_after: HashMap<&str, [i32;2]> = HashMap::new();
        'mai: for i in content1.lines() {
            let mut total = 0;
            let mut idle = 0;
            let mut name = "";
            for (j, k) in i.split_whitespace().enumerate() {
                if j == 0 && ( k == "intr"
                    || k == "page"
                    || k == "swap"
                    || k == "disk_io" 
                    || k == "ctxt" 
                    || k == "btime"
                    || k == "processes"
                    || k == "procs_running"
                    || k == "procs_blocked"
                    || k == "softirq" ) {
                    break 'mai;
                }
                if j == 0 {
                    name = k;
                } else {
                    total += k.parse::<i32>().unwrap();
                }
                if j % 10 == 4 {
                    idle = k.parse::<i32>().unwrap();
                }
            }
            set_after.insert(
                name,
                [total, idle]);
        }
        
        let mut v: Vec<f32> = Vec::new();

        for c in 0..set_before.len() {
            let mut i_b = 0.0;
            let mut t_b = 0.0;
            let mut i_a = 0.0;
            let mut t_a = 0.0;
            let mut key = format!("cpu{}", &c);
            if c == 8 {
                key = "cpu".to_string();
            }
            let a = set_after.get(&*key).unwrap();
            i_a = a[1] as f32;
            t_a = a[0] as f32;
            let b = set_before.get(&*key).unwrap();
            i_b = b[1] as f32;
            t_b = b[0] as f32;

            let calculation = 100.0 * (((t_a - i_a) - (t_b - i_b)) / ( t_a - t_b ));
            v.push(calculation);
        }

        self.cpu_s = v;
    }


    // format -> ["MemTotal", "MemFree", "MemAvailable", "Cached", "SwapTotal", "SwapFree"]
    // in "kB"
    fn mem(&mut self) {
        let path = "/proc/meminfo".to_string();
        let content = read_file(&path).unwrap();

        let mut v: Vec<i32> = Vec::new();
        for i in content.lines() {
            let tem: Vec<&str> = i.split(":").collect();
            if tem[0] == "MemTotal"
                || tem[0] == "MemFree"
                || tem[0] == "MemAvailable"
                || tem[0] == "Cached"
                || tem[0] == "SwapTotal"
                || tem[0] == "SwapFree" {
                let x = tem[1].trim_start().trim_end_matches("kB").trim_end().parse::<i32>().unwrap();
                v.push(x);
            }
        }

        self.mem_s = v;
    }

    // ---------- INCORRECT VALUE ---------------
    fn i_o(&mut self) {
        let path = "/proc/diskstats".to_string();
        let content = read_file(&path).unwrap();

        let mut read_b: Vec<f32> = Vec::new();
        let mut write_b: Vec<f32> = Vec::new();
        'mai: for i in content.lines() {
            for (x,y) in i.split_whitespace().enumerate() {
                if x == 2 && !y.starts_with("nvme0") {
                    continue 'mai;
                } else if x == 5 {
                    read_b.push(y.parse::<f32>().unwrap());
                } else if x == 9 {
                    write_b.push(y.parse::<f32>().unwrap());
                }
            }
        }

        let sec = 0.0;

         // sleep for 5 sec.
        let second = time::Duration::from_secs(sec as u64);
        thread::sleep(second);

        let content1 = read_file(&path).unwrap();

        let mut read_a: Vec<f32> = Vec::new();
        let mut write_a: Vec<f32> = Vec::new();
        'mai: for i in content1.lines() {
            for (x,y) in i.split_whitespace().enumerate() {
                if x == 2 && !y.starts_with("nvme0") {
                    continue 'mai;
                } else if x == 5 {
                    read_a.push(y.parse::<f32>().unwrap());
                } else if x == 9 {
                    write_a.push(y.parse::<f32>().unwrap());
                }
            }
        }

        let mut cal_r: Vec<f32> = Vec::new();
        let mut cal_w: Vec<f32> = Vec::new();
        // in kB/
        for i in 0..read_a.len() {

            cal_r.push((((read_a[i] - read_b[i]) / sec) * 512.0) / 1024.0);
            cal_w.push((((write_a[i] - write_b[i]) / sec) * 512.0) / 1024.0);
        }
        println!("r: {:?}kB : w: {:?}kb",cal_r, cal_w);
    }

    pub fn call_s(&mut self) {
        &self.inet();
        &self.cpu();
        &self.mem();
        &self.uptime();
        &self.i_o();
    }

    pub fn process_nos(&self) -> &i32 {
        &self.process_nos
    }

    pub fn cpu_s(&self) -> &Vec<f32> {
        &self.cpu_s
    }

    pub fn mem_s(&self) -> &Vec<i32> {
        &self.mem_s
    }

    pub fn uptime(&self) -> &String {
        &self.uptime
    }

    pub fn net(&self) -> &Vec<String> {
        &self.net
    }

    pub fn io_s(&self) -> &Vec<i32> {
        &self.io_s
    }
}







