use std::collections::HashMap;
use std::io;
use std::fs;
use std::io::Read;

/*
 * -------------- <TODO>-----------
 *
 * 1. Read folder recursivley   [option]:crate: walkdir:
 * 2. make a function to read process with PID
 * 3. make another function for system wide reading
 * 4. short out which file and folder to read
 * 5. Where to find CPU usage data of per process
 *
 */

const _list: [&str; 8] = ["maps", "numa_maps", "oom_score_adj", "smaps", "stat", "status", "syscall", "task/"];

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

    // testing 
    let readfile  = fs::File::open("/proc/1/status")?;
    let mut buff = io::BufReader::new(readfile);
    let mut content = String::new();
    buff.read_to_string(&mut content)?;
  //  println!("{:?}", content);

    // here awe are sorting 'status'
    let mut fields: HashMap<&str, &str> =  HashMap::new();
    for i in content.split("\n") {
        let v: Vec<&str> = i.split("\t").collect();
        if v[0] == "" {
            continue;
        }
        //println!("{:?}, ----{:?}", v[0], v[1]);
        fields.insert(
            &v[0].trim_end_matches(":"),
            &v[1],
        );

    }
    println!("{:#?}", fields);
    //println!("dir is : {:?} \n and no. of process : {:?}", &process_no, &process_no.len());

    Ok(())
}
