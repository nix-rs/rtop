use std::path::Path;
use std::{fs, vec};
use std::io::{self, ErrorKind};
use std::collections::HashMap;
use std::io::Read;
use std::{
    thread,
    time,
};
use std::process::Command;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Data {
    I32(i32),
    F32(f32),
    S(String),
}

// here we getting all process stat at once
// format -->
// [[pid, state, name, command, thread, user, ppid, mem, cpu], [...], ...]
pub fn all_process() -> Vec<Vec<Data>> {
    let mut main_vec: Vec<Vec<Data>> = Vec::new();
    let mut count = 0;
    let mut cpu = to_cal_cpu();

    if cpu.len() < processes().len() {
        for i in 0..(processes().len() - cpu.len()) {
            cpu.push(0.0);
        }
    }

    for _ in processes().iter() {
        let mut inner_vec: Vec<Data> = Vec::new();
        let mut process = Process::new(*i);
        process.call_p();
        inner_vec.push(Data::I32(process.pid().clone()));
        inner_vec.push(Data::S(process.state().clone()));
        inner_vec.push(Data::S(process.name().clone()));
        inner_vec.push(Data::S(process.command().clone()));
        inner_vec.push(Data::I32(process.threads().clone()));
        inner_vec.push(Data::S(process.user().clone()));
        inner_vec.push(Data::I32(process.ppid().clone()));
        inner_vec.push(Data::I32(process.mem().clone()));
        inner_vec.push(Data::F32(cpu[count as usize]));

        main_vec.push(inner_vec);
        count += 1;
    }
    main_vec
}

pub fn to_cal_cpu() -> Vec<f32> {
    let mut before: Vec<f32> = Vec::new();
    let mut after: Vec<f32> = Vec::new();
    let mut before_total: Vec<f32> = Vec::new();
    let mut after_total: Vec<f32> = Vec::new();

    for i in processes().iter() {
        let mut process_1 = Process::new(*i);
        before_total.push(process_1.cpu_stat()[0]);
        before.push(process_1.cpu_stat()[1]);
    }
    let second = time::Duration::from_millis(300);
    thread::sleep(second);

    for i in processes().iter() {
        let mut process_2 = Process::new(*i);
        after_total.push(process_2.cpu_stat()[0]);
        after.push(process_2.cpu_stat()[1]);
    }

    let mut return_data: Vec<f32> = Vec::new();

    if after.len() != before.len() {
        let dif = after.len().abs_diff(before.len());
        if after.len() > before.len() {
            for _ in 0..dif {
                before.push(0.0);
                before_total.push(0.0);
            }
        } else {
            for _ in 0..dif {
                after.push(0.0);
                after_total.push(0.0);
            }
        }
    }

    for i in 0..after.len() {
        let c = 100.0 * (after[i] - before[i]) / (after_total[i] - before_total[i]);
        return_data.push(c.abs());
    }

    return_data
}

pub fn cal_disk_used() -> Vec<i32> {
    let command = Command::new("df")
        .arg("-h")
        .arg("--output=target,avail")
        .output()
        .expect("lslogins is not working !!!");
    
    let mut data: Vec<i32> = Vec::new();

    let mut strs = String::new();
    for i in command.stdout.iter() {
        let temp = *i as char;
        strs.push(temp);
    }
    
    for i in strs.lines() {
        let temp: Vec<&str> = i.split_whitespace().collect();
        if temp[0] == "/" || temp[0] == "/home" || temp[0] == "/boot/efi" {
            let free_space = temp[1].trim_end_matches(|c| c == 'G' || c == 'M').parse::<i32>()
                .expect("Unable to parse space into i32. ERROR_fn: cal_disk_used()");
            data.push(free_space);
        }
    }
    data
}

// Output -> ['/', '/boot/efi', '/home']
// e.g "/dev/nvme0n1p7" for home
pub fn find_partitions() -> Vec<String> {
    let path = "/proc/mounts".to_string();
    let content = read_file(&path)
        .expect("Couldn't find the path. ERROR_fn: 'find_partitions()_data.rs_L:108'");
    let mut data: Vec<String> = Vec::new();

    for i in content.lines() {
        let mut inner_vec: Vec<&str> = Vec::new();
        for (x, y) in i.split_whitespace().enumerate() {
            if x == 0 || x == 1 {
                inner_vec.push(y);
            }
        }
        if inner_vec[1] == "/" || inner_vec[1] == "/home" || inner_vec[1] == "/boot/efi" {
            data.push(inner_vec[0].trim().to_string());
        }
    }
    //println!("{:?}", data);
    data
}

// here we are getting UID to users.
// This function is bottleneck
// Output format -> ("UID": "user_name");
pub fn users() -> HashMap<String, String> {
    let command = Command::new("lslogins")
        .output()
        .expect("lslogins is not working !!!");

    // converting u8 into string
    let mut strs = String::new();
    for i in command.stdout.iter() {
        let temp = *i as char;
        strs.push(temp);
    }

    let mut output: HashMap<String, String> = HashMap::new();
    for (i, j) in strs.split("\n").enumerate() {
        let inner_vec: Vec<&str> = j.split_whitespace().collect();
        if inner_vec.is_empty() || i == 0 {
            continue;
        }
        output.insert(
            inner_vec[0].to_string(),
            inner_vec[1].to_string(),
        );
    }
    output
}

// here we are saving all the running processes
pub fn processes() -> Vec<i32> {
    let mut process_no: Vec<i32> = Vec::new();
    let paths = fs::read_dir("/proc")
        .expect("Couldn't read the number of processes from /proc dir. ERROR_fn: porcesses()_data.rs_L:164");

    for path in paths {
        let check = path
            .expect("Couldn't get the DirEntry. ERROR_fn: processes()_data.rs_L:168")
            .file_name()
            .into_string()
            .expect("Couldn't convert the DirEntry into string. ERROR_fn: processes()_data.rs_L:171");
        if check.parse::<i32>().is_ok() {
            process_no.push(check.parse::<i32>().unwrap())
        }
    }
    process_no
}

// for reading files for given a path
fn read_file(path : &String) -> Result<String, io::Error> {
    let readfile  = fs::File::open(path)?;
    let mut buffer = io::BufReader::new(readfile);
    let mut content = String::new();
    buffer.read_to_string(&mut content)?;

    Ok(content)
}

/// Here we are reading the main 'stat' file to get the total time
/// of all CPUs cumulatively
fn main_cpu_ticks()  -> f32 {
    let path = "/proc/stat".to_string();
    let stats = read_file(&path)
        .expect("Couldn't read the /proc/stat. ERROR_fn: main_cpu_ticks()_data.rs_L:192");

    let mut ticks: f32 = 0.0;

    for i in stats.lines() {
        for j in i.split(" ") {
            //println!("{j}");
            if j.parse::<i32>().is_ok() {
                let tick = j.parse::<f32>()
                    .expect("Couldn't parse the ticks into float. ERROR_fn: main_cpu_ticks()_data.rs_L:201");
                ticks += tick;
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

    // starting all the functions at once
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
    pub fn status(&mut self) {
        let path = format!("/proc/{}/status", self.pid);
        let content = match read_file(&path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => "".to_string(),
                _ => panic!("Couldn't find the /proc/pid/status file"),
            },
        };
        let mut fields: HashMap<&str, &str> = HashMap::new();

        for i in content.split("\n") {
            let inner_vec: Vec<&str> = i.split("\t").collect();
            if inner_vec[0] == "" {
                continue;
            }
            if inner_vec[0] == "Name:" 
                || inner_vec[0] == "State:" 
                || inner_vec[0] == "PPid:" 
                || inner_vec[0] == "Threads:" 
                || inner_vec[0] == "VmRSS:" 
                || inner_vec[0] == "Uid:" {
                if inner_vec[0] == "VmRSS:" {
                    fields.insert(
                        inner_vec[0].trim_end_matches(":"),
                        inner_vec[1].trim_start().trim_end_matches("kB").trim_end(),
                    );
                } else {
                    fields.insert(
                        inner_vec[0].trim_end_matches(":"),
                        inner_vec[1]
                    );
                }
            }
        }

        self.name = fields
            .get("Name")
            .unwrap_or(&"n")
            .to_string();
        self.state = (*fields.get("State").unwrap_or(&"s")).to_string();
        self.ppid = (*fields.get("PPid").unwrap_or(&"0")).parse::<i32>().unwrap();
        self.threads = (*fields.get("Threads").unwrap_or(&"0")).parse::<i32>().unwrap();
        self.mem = (*fields.get("VmRSS").unwrap_or(&"0")).parse::<i32>().unwrap();
        self.user = (*fields.get("Uid").unwrap_or(&"0")).to_string();
    }

    /// Here we are reading per process cpu usages
    /// * CPU%
    fn cpu_stat(&mut self) -> Vec<f32> {
        let mut to_send: Vec<f32> = Vec::new();
        to_send.push(main_cpu_ticks());
        let mut utime_before: f32 = 0.0;

        let path = format!("/proc/{}/stat", self.pid);
        let content = match read_file(&path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => "".to_string(),
                _ => panic!("Couldn't find the /proc/pid/stat file."),
            },
        };

        for (i, j) in content.split(" ").enumerate() {
            if i == 13 {
                utime_before = j.parse::<f32>().unwrap();
            }
        }
        to_send.push(utime_before);

        to_send
    }

    /// * command
    fn cmdline(&mut self) {
        let path = format!("/proc/{}/cmdline", self.pid);
        let content = match read_file(&path) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => "".to_string(),
                _ => panic!("Couldn't find the /proc/pid/cmdline file."),
            },
        };

        if content != "".to_string() {
            self.command = content;
        } else {
            self.command = "".to_string();
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
    io_s: Vec<f32>,
    net: Vec<String>,
    uptime: String,
    //version: String,
    mem_s: Vec<i32>,
    battery: Vec<String>,
    cpu_speed_n_info: Vec<String>,
    cpu_cores: i32,
    cpu_temp: Vec<f32>,
    disk_stat: Vec<f32>
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
            battery: Vec::new(),
            cpu_speed_n_info: Vec::new(),
            cpu_cores: 0,
            cpu_temp: Vec::new(),
            disk_stat: Vec::new(),
        }
    }

    // calculating the uptime of System
    // Output format -> "00:00:00"
    fn uptime_c(&mut self) {
        //let pathV = "/proc/version".to_string();
        let path = "/proc/uptime".to_string();
        //let contentV = read_file(&pathV).unwrap();
        let content = read_file(&path)
            .expect("Couldn't read the /proc/uptime. ERROR_fn: uptime_c()_data.rs_L:422");
        let times: Vec<&str> = content.split(" ").collect();
        //println!("V: '{}'", v[0]);
        let uptime = times[0].parse::<f32>()
            .expect("Couldn't parse the times into f32. ERROR_fn: uptime_c()_data.rs_L:426");

        let sec =&uptime % 60.0;
        let min = ((&uptime - sec) / 60.0) % 60.0;
        let hr = (((&uptime - sec) / 60.0) - min ) / 60.0;

        self.uptime = format!("{0:02}:{1:02}:{2:02}", hr, min, sec.round());
    }

    // Output format -> ["dev_name", "recv bytes", "recv / s", "send_bytes", "send / s ...]
    pub fn inet(&mut self) {
        let path = "/proc/net/dev".to_string();
        let content = read_file(&path)
            .expect("Couldn't read the /proc/net/dev. ERROR_fn: inet()_data.rs_L:439");

        let mut data: Vec<String>= Vec::new();
        let mut before: Vec<String> = Vec::new();
        let mut after: Vec<String> = Vec::new();

        for (i, j) in content.split("\n").enumerate() {
            if i == 0 || i == 1 {
                continue
            }
            for (x, y) in j.split_whitespace().enumerate() {
                if x == 0 || x == 1 || x == 9 {
                    before.push(y.to_string());
                }
            }
        }

        thread::sleep(time::Duration::from_secs(1));

        let content = read_file(&path)
            .expect("Couldn't read the /proc/net/dev. ERROR_fn: inet()_data.rs_L:439");

        for (i, j) in content.split("\n").enumerate() {
            if i == 0 || i == 1 {
                continue
            }
            for (x, y) in j.split_whitespace().enumerate() {
                if x == 0 || x == 1 || x == 9 {
                    after.push(y.to_string());
                }
            }
        }

        for i in 0..after.len() {
            if i == 0 || i % 3 == 0 {
                data.push(before[i].clone());
                continue;
            }

            if i == 1 || i % 3 == 1 {
                data.push(after[i].clone());
            }

            if i == 2 || i % 3 == 2 {
                data.push(after[i].clone());
            }

            let after = after[i].parse::<i32>()
                .expect("Unable to parse into i32. ERROR_fn: inet()_data.rs_L:487");

            let before = before[i].parse::<i32>()
                .expect("Unable to parse into i32. ERROR_fn: inet()_data.rs_L:490");

            let diff = after - before;

            data.push(diff.to_string());
        }
        self.net = data;
    }

    // Output format -> [cpu0, cpu1, cpu2, cpu3, cpu4, cpu5, cpu6, cpu7, cpu]
    // in '%'
    pub fn cpu(&mut self) {
        let mut set_before: HashMap<&str, [i32;2]> = HashMap::new();
        let path = "/proc/stat".to_string();
        let content = read_file(&path)
            .expect("Couldn't read the /proc/stat. ERROR_fn: cpu()_data.rs_L:462");

        'm: for i in content.lines() {
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
                    break 'm;
                }
                if j == 0 {
                    name = k;
                } else {
                    total += k.parse::<i32>()
                        .expect("Couldn't parse cpu total ticks into i32. ERROR_fn: cpu()_data.rs_L:485");
                }
                if j % 10 == 4 {
                    idle = k.parse::<i32>()
                        .expect("Couldn't parse cpu idle ticks into i32. ERROR_fn: cpu()_data.rs_L:489");

                }
            }
            set_before.insert(
                name,
                [total, idle]);
        }

        // sleep for 250 millis.
        let second = time::Duration::from_millis(250);
        thread::sleep(second);

        let path = "/proc/stat".to_string();
        let content1 = read_file(&path)
            .expect("Couldn't read the /proc/stat. ERROR_fn: cpu()_data.rs_L:504");

        let mut set_after: HashMap<&str, [i32;2]> = HashMap::new();
        'm: for i in content1.lines() {
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
                    break 'm;
                }
                if j == 0 {
                    name = k;
                } else {
                    total += k.parse::<i32>()
                        .expect("Couldn't parse cpu total ticks into i32. ERROR_fn: cpu()_data.rs_L:528");
                }
                if j % 10 == 4 {
                    idle = k.parse::<i32>()
                        .expect("Couldn't parse cpu total ticks into i32. ERROR_fn: cpu()_data.rs_L:533");
                }
            }
            set_after.insert(
                name,
                [total, idle]);
        }

        let mut data: Vec<f32> = Vec::new();

        for c in 0..set_before.len() {
            let mut idle_before = 0.0;
            let mut total_before = 0.0;
            let mut idle_after = 0.0;
            let mut total_after = 0.0;
            let mut key = format!("cpu{}", &c);
            if c == 8 {
                key = "cpu".to_string();
            }
            let after = set_after.get(&*key)
                .expect("Couldn't get the key in 'set_after'. ERROR_fn: cpu()_data.rs_L:552");
            idle_after = after[1] as f32;
            total_after = after[0] as f32;
            let before = set_before.get(&*key)
                .expect("Couldn't get the key in 'set_before'. ERROR_fn: cpu()_data.rs_L:556");

            idle_before = before[1] as f32;
            total_before = before[0] as f32;

            let calculation = 100.0 * (((total_after - idle_after) - (total_before - idle_before)) / ( total_after - total_before ));
            data.push(calculation);
        }
        self.cpu_s = data;
    }

    // Output format -> ["MemTotal", "MemFree", "MemAvailable", "Cached", "SwapTotal", "SwapFree"]
    // in "kB"
    fn mem(&mut self) {
        let path = "/proc/meminfo".to_string();
        let content = read_file(&path)
            .expect("Couldn't read the /proc/meminfo. ERROR_fn: mem()_data.rs_L:573");

        let mut data: Vec<i32> = Vec::new();
        for i in content.lines() {
            let temp: Vec<&str> = i.split(":").collect();
            if temp[0] == "MemTotal"
                || temp[0] == "MemFree"
                || temp[0] == "MemAvailable"
                || temp[0] == "Cached"
                || temp[0] == "SwapTotal"
                || temp[0] == "SwapFree" {
                let push = temp[1].trim_start().trim_end_matches("kB").trim_end().parse::<i32>()
                    .expect("Couldn't parse the memory into i32. ERROR_fn: mem()_data.rs_L:585");
                data.push(push);
            }
        }
        self.mem_s = data;
    }

    // Output format -> [efi_read, efi_write, boot_read, boot_write, home_read, home_write]
    pub fn i_o(&mut self) {
        let path = "/proc/diskstats".to_string();
        let before_content = read_file(&path)
            .expect("Couldn't read the /proc/diskstats. ERROR_fn: i_o()_data.rs_L:596");

        let disks = find_partitions();
        let mut disk: Vec<&str> = Vec::new();
        for i in disks.iter() {
            let temp: Vec<&str> = i.split("/").collect();
            disk.push(temp[2]);
        }
        //println!("{:?}", disk);

        let mut read_before: Vec<f32> = Vec::new();
        let mut write_before: Vec<f32> = Vec::new();
        for i in before_content.lines() {
            let mut temp: Vec<&str> = i.split_whitespace().collect();
            if temp[2] == disk[0] || temp[2] == disk[1] || temp[2] == disk[2] {
                read_before.push(temp[5].parse::<f32>()
                    .expect("Couldn't parse temp into f32. ERROR_fn: i_o()_data.rs_L:612"));
                write_before.push(temp[9].parse::<f32>()
                    .expect("Couldn't parse temp into f32. ERROR_fn: i_o()_data.rs_L:614"));
            }
        }

        let sec = 1.0;
        // sleep for 5 sec.
        let second = time::Duration::from_secs(sec as u64);
        thread::sleep(second);

        let after_content = read_file(&path)
            .expect("Couldn't read the /proc/diskstats. ERROR_fn: i_o()_data.rs_L:624");

        let mut read_after: Vec<f32> = Vec::new();
        let mut write_after: Vec<f32> = Vec::new();
        for i in after_content.lines() {
            let mut temp: Vec<&str> = i.split_whitespace().collect();
            if temp[2] == disk[0] || temp[2] == disk[1] || temp[2] == disk[2] {
                read_after.push(temp[5].parse::<f32>()
                    .expect("Couldn't parse temp into f32. ERROR_fn: i_o()_data.rs_L:632"));
                write_after.push(temp[9].parse::<f32>()
                    .expect("Couldn't parse temp into f32. ERROR_fn: i_o()_data.rs_L:634"));
            }
        }

        let mut data: Vec<f32> = Vec::new();
        // in kB/
        for i in 0..read_after.len() {
            data.push((((read_after[i] - read_before[i]) / sec) * 512.0) / 1024.0);
            data.push((((write_after[i] - write_before[i]) / sec) * 512.0) / 1024.0);
        }
       self.io_s = data;
    }

    // calculating no. of processes
    fn pro_no(&mut self) {
        self.process_nos = processes().len() as i32;
    }

    // Output format -> ["Status", "Charged %"]
    pub fn battery_s(&mut self) {
        let path = "/sys/class/power_supply/BAT1/uevent".to_string();
        let content = read_file(&path)
            .expect("Couldn't read the /sys/class/power_supply/BAT1/uevent. ERROR_fn: battery_s()_data.rs_L:656");

        let mut data: Vec<String> = Vec::new();

        for i in content.lines() {
            let temp: Vec<&str> = i.split("=").collect();
            if temp[0] == "" || temp[1] == "" {
                continue;
            } else if temp[0] == "POWER_SUPPLY_STATUS" || temp[0] == "POWER_SUPPLY_CAPACITY" {
                data.push(temp[1].to_string());
            }
        }
        self.battery = data;
    }

    /// BUG -- only specilized for Intel CPUs
    ///     -- also we are calculating no. of cores again
    // here we are capturing cpu speed[MHz] and cpu name on String
    // Output format -> [ mHz0, mHz1, mHz2, mHz3, mHz4, mHz5, mHz6, mHz7, "cpu_name"]
    fn cpu_info(&mut self) {
        let path = "/proc/cpuinfo".to_string();
        let content = read_file(&path)
            .expect("Couldn't read the /pro/cpuinfo. ERROR_fn: cpu_info()_data.rs_L:678");

        let mut cpu_cores = 0;
        let mut model_name = "";
        let mut data: Vec<String> = Vec::new();
        for i in content.lines() {
            let temp: Vec<&str> = i.split(":").collect();
            if temp[0] == "" || temp[1] == "" {
                continue;
            } else if temp[0] == "model name\t" {
                model_name = temp[1];
            } else if temp[0] == "cpu MHz\t\t" {
                data.push(temp[1].trim().to_string());
            } else if temp[0] == "cpu cores\t" {
                cpu_cores = temp[1].trim().parse::<i32>()
                    .expect("Couldn't parse the cores into i32. ERROR_fn: cpu_info()_data.rs_L:693");
            }
        }
        data.push(model_name.trim().to_string());
        self.cpu_cores = cpu_cores;
        self.cpu_speed_n_info = data;
        //println!("{:?}> {}", data, cpu_cores);
    }

    // Output format->> [main, core0, core1, core2, core3]
    // in celcius
    pub fn cpu_t(&mut self) {
        let mut path = "".to_string();
        for i in 0..=10 {
            let pat = format!("/sys/devices/platform/coretemp.0/hwmon/hwmon{}/", i);
            let paths = Path::new(&pat);
            if paths.is_dir() {
                path = pat;
            }
        }
        let cores = self.cpu_cores;
        let mut data: Vec<f32> = Vec::new();
        for i in 1..=cores + 1 {
            let content = read_file(&(format!("{}temp{}_input",path, i )).to_string())
                .expect("Couldn't read the temp file. ERROR_fn: cpu_t()_data.rs_L:717");
            //println!("{}", content);
            let temp = content.trim().parse::<f32>()
                .expect("Couldn't parse the temp into f32. ERROR_fn: cpu_t()_data.rs_L:720");
            data.push(temp/1000.0);
        }

        //println!("{:?}", data);
        self.cpu_temp = data;
    }

    /// --- BUG ---
    /// here we are only working with SSD system improve for HDD
    /// and for other combinations of partinoned letter combinations
    /// -----here read_file "unwrap" should be more flexible
    // Output format -> [/total, /used, /efi_total, /efi_used, /home_total, /home_used]
    //           [  gb,    gb,       gb,         gb,        gb,          gb,    ]
    pub fn disk_st(&mut self) {
        let part_name = find_partitions();

        let path_n: Vec<&str> = part_name[0].split("/").collect();
        let path_n: Vec<&str> = path_n[2].split("0").collect();
        let path_n = path_n[0];
    
        let path = format!("/sys/class/{path_n}/{path_n}0/{path_n}0n1/");

        let mut data: Vec<f32> = Vec::new();

        for i in part_name.iter() {
            let temp: Vec<&str> = i.split("/").collect();
            let path = format!("{}{}/", path, temp[2]);
            let to_read = vec!["size".to_string(), "stat".to_string()];
            //println!("{}", path);
            for j in to_read.iter() {
                let content = read_file(&format!("{}{}", path, j))
                    .expect("Couldn't read the partitoned paths. ERROR_fn: disk_st()_data.rs_L:752");
                //println!("{}", content);
                if j == "size" {
                    let disk_size = content.trim().parse::<f32>()
                        .expect("Couldn't parse the disk data into f32. ERROR_fn: disk_st()_data.rs_L:756");
                    let disk_size = (((disk_size * 512.0) / 1024.0) / 1024.0 ) / 1024.0;
                    data.push(disk_size);
                } else if j == "stat" {
                    for (m, n) in content.split_whitespace().enumerate() {
                        if m == 13 {
                            let used_size = n.trim().parse::<f32>()
                                .expect("Couldn't parse the used size into f32. ERROR_fn: disk_st()_data.rs_L:763");
                            let used_size = (((used_size * 512.0) / 1024.0) / 1024.0) / 1024.0;
                            data.push(used_size);
                        }
                    }
                }
            }
        }
        self.disk_stat = data;
    }


    // starting all the fn's
    pub fn call_s(&mut self) {
        &self.inet();
        &self.cpu();
        &self.mem();
        &self.uptime_c();
        &self.i_o();
        &self.pro_no();
        &self.cpu_info();
        &self.battery_s();
        &self.cpu_t();
        &self.disk_st();
    }

    pub fn cpu_cores(&self) -> &i32 {
        &self.cpu_cores
    }

    pub fn disk_stat(&self) -> &Vec<f32> {
        &self.disk_stat
    }

    pub fn cpu_speed_n_info(&self) -> &Vec<String> {
        &self.cpu_speed_n_info
    }

    pub fn cpu_temp(&self) -> &Vec<f32> {
        &self.cpu_temp
    }

    pub fn battery(&self) -> &Vec<String> {
        &self.battery
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

    pub fn io_s(&self) -> &Vec<f32> {
        &self.io_s
    }
}

