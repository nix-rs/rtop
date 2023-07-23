use std::io;
use std::fs;


fn main() {

    let paths = fs::read_dir("/proc/1").unwrap();

    for path in paths{
        println!("Name is : {}", path.unwrap().path().display());
    }

}
