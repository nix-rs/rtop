#![allow(unused)]
use std::{
    io::{self, Read, stdout, Write, ErrorKind, stdin, Stdin},
    time, thread, sync::{Arc, Mutex}, collections::HashMap,
};

pub mod data;
mod ui;

use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::{IntoAlternateScreen, AlternateScreen},
    cursor::{Hide, Goto},
    input::TermRead,
    event::Key, terminal_size, async_stdin,
    clear,
};

use crate::ui::{
    Ui,
    Position,
};

use crate::data::{
    Process,
    System,
    all_process,
    Data,
    find_partitions,
    users,
    processes,
    to_cal_cpu,
    cal_disk_used,
};

/// -- Future Impl
///  * sorting the processes
///  * set the custom refresh time
///  * sorting of Process
///  * implement threading for accurate data like
///     - some time porcess numbers didnt mathes with data
///       because of time gap.
///  * CPU UI for octa core
///  * Get rid of Clone().

/// BUGS
/// * DATA Bugs
///             - show the linux version
///             - day impl on Uptime
///             - remove the disk free calculation from disk_stat
///             - we are not using the CPU name
/// * ui        - key indicator
///             - color net
///             - add more rows to 'ui_other'
/// * thread handling of some routine

fn main() -> io::Result<()> {

    let stdin = async_stdin();
    let mut stdout = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
    write!(stdout, "{}", termion::cursor::Hide);

    let uid = users();
    let free_space = cal_disk_used();
    let mut uii = Ui::new(stdout, stdin, uid, free_space);
    uii.start();

    //let mut sys = System::new();
    //sys.call_s();
    
    //let s = cal_disk_used();
    //println!("{:?}", s);

    Ok(())
}

/*
fn main() -> io::Result<()> {
    thread::scope(|s| {
        let stdin = async_stdin();
        let mut stdout = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
        write!(stdout, "{}", termion::cursor::Hide);
        let uid = users();
        let mut  uii = Ui::new(stdout, stdin, uid);
        //uii.start();
        let mut sys = System::new();

        s.spawn(move || {
            sys.call_s();
            uii.ui_io(sys.disk_stat(), sys.io_s());
            //self.ui_mem(&sys.mem_s());
            //self.ui_net();
            //self.ui_cpu(sys.cpu_s(), sys.cpu_temp(), sys.cpu_speed_n_info(), sys.cpu_cores());
            //self.ui_other(sys.uptime(), sys.battery(), sys.process_nos());
            //self.stdout.flush().unwrap();
            //write!(self.stdout, "{}", clear::All);
            //let second = time::Duration::from_secs(1);
            //thread::sleep(second);


        });

        s.spawn( move|| {
            loop {
                if let Err(e) = uii.key() {
                    panic!("oppps: {}", e);
                }
                uii.ui();
                uii.ui_proces();
        }
        } );


    Ok(())

    })
}
*/

/*
    pub fn start(&mut self) {
        let mut sys = System::new();
        loop {
            thread::scope(|s| {
                //sys.call_s();
                //s.spawn(|| {

                //})
            });
            //sys.call_s();
            //thread::scope(|scope| {
                if let Err(e) = self.key() {
                    panic!("oops {}", e);
                }
                self.ui_proces();
           // });

            self.ui();
            //self.ui_io(sys.disk_stat(), sys.io_s());
            //self.ui_mem(&sys.mem_s());
            //self.ui_net();
            //self.ui_cpu(sys.cpu_s(), sys.cpu_temp(), sys.cpu_speed_n_info(), sys.cpu_cores());
            //self.ui_other(sys.uptime(), sys.battery(), sys.process_nos());
            self.stdout.flush().unwrap();
            write!(self.stdout, "{}", clear::All);
            //let second = time::Duration::from_secs(1);
            //thread::sleep(second);

        }
    }
*/

