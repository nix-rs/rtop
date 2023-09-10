use std::{
    io::{Write, Read}, process,
    collections::HashMap, 
    ffi::OsStr,},
};
use termion::{
    color,
    cursor::{self,Goto}, terminal_size,
    clear, style,
};
use crate::data::{
    all_process,
    Data,
    System,
};


#[derive(Debug, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

/// SYNOPSYS :
/// r - row
/// c - column
///
impl Position {
    pub fn new() -> Self {
        Self {
            x: 1, 
            y: 1
        }
    }

    pub fn screen_size() -> Self {
        let mut size = terminal_size()
            .expect("Couldn't get the terminal size. Error_fn: screen_size()_ui.rs_L:48");
        Self {
            x: size.0,
            y: size.1
        }
    }

    // output -> y
    // ROW DIVIDER 
    // and always set to the row_no: 11 irrespective of screen size
    fn row_p(&mut self) -> u16 {
        let point = 11;
        point
    }

    // output -> x
    // COLUMN DIVIDER
    // relative to s~creen size
    fn col_p(&mut self) -> u16 {
        let point = Self::screen_size().x * 70 / 100;
        point
    }

    // SAME 'process_head_p'
    fn row_cpu(&mut self) -> u16 {
        let point = self.row_p() + 1;
        point
    }

    fn col_start_ii(&mut self) -> u16 {
        let point = self.col_p() + 1;
        point
    }
    
    fn col_other(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.35;
        point as u16
    }

    fn col_net_i(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.55;
        point as u16
    }

    fn col_net_ii(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.67;
        point as u16
    }

    fn col_net_iii(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.77;
        point as u16
    }

    fn col_net_iv(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.85;
        point as u16
    }

    fn col_net_v(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.95;
        point as u16
    }

    fn row_mem(&mut self) -> u16 {
        1
    }

    // Relative
    fn row_io(&mut self) -> u16 {
        let point = Self::screen_size().y as f32 - 6.0;
        point as u16
    }

    // DUPLICATE ----
    fn process_head_p(&mut self) -> u16 {
        let point = self.row_p() + 1;
        point
    }

    // output -> x  --cumulative point
    fn utilised_p(&mut self) -> u16 {
        let point = Self::screen_size().x as f32 - self.col_p() as f32;
        let point = point * 0.4464;
        let point = point + self.col_p() as f32;
        point.round() as u16
    }

    // output -> x --cumulative point
    fn absolute_p(&mut self) -> u16 {
        let point = Self::screen_size().x as f32 - self.col_p() as f32;
        let point = point * 0.8036;
        let point = point + self.col_p() as f32;
        point.round() as u16
    }

    // output -> x --cumulative
    fn temp_p(&mut self) -> u16 {
        let point = Self::screen_size().x as f32 - self.col_p() as f32;
        let point = point * 0.8571;
        let point = point + self.col_p() as f32;
        point.round() as u16
    }

    // output -> x [6.5%]   (convert these points into cumulative)
    fn pid_p(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.065;
        point.round() as u16
    }

    // output -> x [2.0%]   (convert these points into cumulative)
    fn state_p(&mut self) -> u16 {
        let point = self.col_p() as f32  * 0.02;
        point.round() as u16
    }

    // output -> x [14.3%]  (convert these points into cumulative)
    fn program_p(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.143;
        point.round() as u16
    }

    // output -> x [53%]    (convert these points into cumulative)
    fn command_p(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.53;
        point.round() as u16
    }

    // output -> x [13.4%]  (convert these points into cumulative)
    fn user_p(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.11;
        point.round() as u16
    }

    // output -> x [5.4%]   (convert these points into cumulative)
    fn cpu_p(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.054;
        point.round() as u16
    }

    // output -> x [5.4%]   (convert these points into cumulative)
    fn mem_p(&mut self) -> u16 {
        let point = self.col_p() as f32 * 0.054;
        point.round() as u16
    }
}

enum Direction {
    Up,
    Down,
}

pub struct Ui<R, W> {
    stdin: R,
    stdout: W,
    data_process: Vec<Vec<Data>>,
    cursor_position: Position,
    uid: HashMap<String, String>,
    position: u16,
    to_kill: String,
    to_quit: bool,
    free_space: Vec<i32>,
}


impl<'a, R : Read, W : Write> Ui<R, W> {
    pub fn new(mut stdout: W, stdin: R, uid: HashMap<String , String>, free_space: Vec<i32>) -> Ui<R, W> {
        Ui {
            stdin,
            stdout,
            data_process: all_process(),
            cursor_position: Position::new(),
            uid,
            position: 0,
            to_kill: String::new(),
            to_quit: false,
            free_space,
        }
    }

    fn kill(&mut self) {
        let os_str = OsStr::new(&self.to_kill);
        let _process = process::Command::new("kill")
            .arg(os_str)
            .spawn()
            .expect("Failed to Kill the Process");
    }

    pub fn key(&mut self) -> Result<(), std::io::Error> {
        let mut byte = [0];
        let _k = self.stdin.read(&mut byte)?;

        match byte[0] {
            b'q' | b'Q' => self.to_quit = true,
            b'e' | b'E' => self.kill(),             // kill a process
            b'w' | b'W' => {
                self.position -= 1;
            },
            b's' | b'S' => {
                self.position += 1;
            },
            //b'a' => (),             // sorting up
            //b'd' => (),             // sortind down
            //b'n' => (),             // referesh time up
            //b'm' => (),            // referesh time down
            _ => (),
        }

        Ok(())
    }

    pub fn start(&mut self) {
        let mut sys = System::new();
        loop {
            sys.call_s();

            if let Err(e) = self.key() {
                panic!("oops {}", e);
            }
            self.ui_proces();

            if self.to_quit {
                break;
            }

            self.ui();
            self.ui_io(sys.disk_stat(), sys.io_s());
            self.ui_mem(&sys.mem_s());
            self.ui_net(sys.net());
            self.ui_cpu(sys.cpu_s(), sys.cpu_temp(), sys.cpu_speed_n_info(), sys.cpu_cores());
            self.ui_other(sys.uptime(), sys.battery(), sys.process_nos());
            self.stdout.flush().unwrap();
            write!(self.stdout, "{}", clear::All)
                .expect("Couldn't write! from 'start'. Error_fn: start()_ui.rs_L:271");
            //let second = time::Duration::from_secs(1);
            //thread::sleep(second);
        }
        write!(self.stdout, "{}{}{}{}", clear::All, style::Reset, cursor::Show, Goto(1,1))
            .expect("Couldn't write! from 'start'. Error_fn: start()_ui.rs_L:271");

        self.stdout.flush()
            .expect("Unable to flush screen. Error_fn: start()_ui.rs_L:274");
    }

    pub fn ui_mem(&mut self, data: &Vec<i32>) {
        let mut position = Position::new();
        let row = position.row_mem();
        let col = position.col_start_ii();
        let col_u = position.utilised_p();
        let col_ab = position.absolute_p();

        let data = data;

        let div = 1024.0 * 1024.0;

        let mem_total = data[0];
        let total_gb = mem_total as f64 / div;
        let mem_free = data[1];
        let free_gb = mem_free as f64 / div;
        let mem_ava = data[2];
        let ava_gb = mem_ava as f64 / div;
        let mem_cached = data[3];
        let cached_gb = mem_cached as f64 / div;
        let mem_stotal = data[4];
        let stotal_gb = mem_stotal as f64 / div;
        let mem_sfree = data[5];
        let sfree_gb = mem_sfree as f64 / div;

        let used_gb = total_gb - ava_gb;

        let free_p = 100 * mem_free / mem_total;
        let used_p = 100 * (mem_total - mem_ava) / mem_total;
        let available_p = 100 * mem_ava / mem_total;
        let cached_p = 100 * mem_cached / mem_total;
        let swap_p = 100 * mem_sfree / mem_stotal;

        write!(self.stdout, "{0}{1}Available{2}{3}%{4}{5:.2}GiB",
            color::Fg(color::LightGreen),
            Goto(col,row),
            Goto(col_u, row),
            available_p,
            Goto(col_ab, row),
            ava_gb
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:324");

        write!(self.stdout, "{}{}{}",
            color::Fg(color::LightYellow),
            Goto(col,row + 1),
            Self::per(available_p as f32)
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:330");

        write!(self.stdout, "{0}{1}Used{2}{3}%{4}{5:.2}GiB",
            color::Fg(color::LightGreen),
            Goto(col,row + 2), 
            Goto(col_u, row + 2),
            used_p,
            Goto(col_ab, row + 2),
            used_gb
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:340");

        write!(self.stdout, "{}{}{}",
            color::Fg(color::LightYellow),
            Goto(col,row + 3),
            Self::per(used_p as f32)
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:345");

        write!(self.stdout, "{0}{1}Free{2}{3}%{4}{5:.2}GiB",
            color::Fg(color::LightGreen),
            Goto(col,row + 4),
            Goto(col_u, row + 4),
            free_p,
            Goto(col_ab, row + 4), 
            free_gb
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:354");

        write!(self.stdout, "{}{}{}",
            color::Fg(color::LightYellow),
            Goto(col,row + 5), 
            Self::per(free_p as f32)
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:360");

        write!(self.stdout, "{0}{1}Cached{2}{3}%{4}{5:.2}GiB", 
            color::Fg(color::LightGreen), 
            Goto(col,row + 6),
            Goto(col_u, row + 6),
            cached_p,
            Goto(col_ab, row + 6),
            cached_gb
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:369");
        write!(self.stdout, "{}{}{}",
            color::Fg(color::LightYellow),
            Goto(col,row + 7), 
            Self::per(cached_p as f32)
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:374");

        write!(self.stdout, "{0}{1}Free Swap{2}{3}%{4}{5:.2}GiB",
            color::Fg(color::LightGreen),
            Goto(col,row + 8),
            Goto(col_u, row + 8),
            swap_p,
            Goto(col_ab, row + 8),
            sfree_gb
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:383");
        write!(self.stdout, "{}{}{}",
            color::Fg(color::LightYellow),
            Goto(col,row + 9),
            Self::per(swap_p as f32)
        ).expect("Couldn't write! from 'ui_mem'. Error_fn: ui_mem()_ui.rs_L:388");
    }

    pub fn ui_other(&mut self, uptime: &String, bat: &Vec<String>, pnos: &i32 ) {
        let mut position = Position::new();
        let col = position.col_other();

        let uptime = uptime;
        let bat = bat;
        let process_nos = pnos;
        let color = color::Rgb(255,125,0);
        let mut bat_color = color::Rgb(214, 40, 40);

        if bat[0] == "Charging".to_string() {
            bat_color = color::Rgb(128, 185, 24);
        }

        write!(self.stdout, "{0}{1}Uptime :{2}{3}",color::Fg(color), Goto(1, 2), Goto(col, 2), uptime)
            .expect("Couldn't write! from 'ui_other'. Error_fn: ui_other()_ui.rs_L:401");
        write!(self.stdout, "{}{}Battery: {}{}{}%", color::Fg(color), Goto(1,3), Goto(col,3), color::Fg(bat_color), bat[1] )
            .expect("Couldn't write! from 'ui_other'. Error_fn: ui_other()_ui.rs_L:403");
        write!(self.stdout, "{}{}Process NOS: {}{}", color::Fg(color), Goto(1,4), Goto(col,4), process_nos)
            .expect("Couldn't write! from 'ui_other'. Error_fn: ui_other()_ui.rs_L:407");
        write!(self.stdout, "{}{}KEYS: Up [w], down [s], Kill [e], Quit [q]", color::Fg(color), Goto(1, 10))
            .expect("Couldn't write! from 'ui_other'. Error_fn: ui_other()_ui.rs_L:405");

    }

    fn per(val: f32) -> &'a str {
        let mut bar = "";
        if val >= 0.0 && val <= 10.0 {
            //bar = "████";
            bar = "▀";
            return bar
        } else if val >= 10.0 && val <= 20.0 {
            //bar = "████████";
            bar = "▀▀▀▀▀▀▀▀▀▀";
            return bar
        }else if val >= 20.0 && val <= 30.0 {
            //bar = "████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        }else if val >= 30.0 && val <= 40.0 {
            //bar = "████████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        }else if val >= 40.0 && val <= 50.0 {
            //bar = "████████████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        }else if val >= 50.0 && val <= 60.0 {
            //bar = "████████████████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        }else if val >= 60.0 && val <= 70.0 {
            //bar = "████████████████████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        }else if val >= 70.0 && val <= 80.0 {
            //bar = "████████████████████████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        }else if val >= 80.0 && val <= 90.0 {
            //bar = "████████████████████████████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        } else {
            //bar = "████████████████████████████████████████";
            bar = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
            return bar
        }
    }

    pub fn ui_net(&mut self, data: &Vec<String>) {
        let mut position = Position::new();
        let row = 1;
        let col_i = position.col_net_i();
        let col_ii = position.col_net_ii();
        let col_iii = position.col_net_iii();
        let col_iv = position.col_net_iv();
        let col_v = position.col_net_v();

        write!(self.stdout, "{}devices {}download {}ps {}upload {}ps",
            Goto(col_i, row),
            Goto(col_ii , row),
            Goto(col_iii, row),
            Goto(col_iv, row),
            Goto(col_v, row),
        ).expect("Couldn't write! from 'ui_net'. Error_fn: ui_net()_ui.rs_L:494");

        let data = data;
        let mut row = 2;
        for (i,j) in data.iter().enumerate() {
            if i == 0 || i % 5 == 0 {
                write!(self.stdout, "{}{}", Goto(col_i, row + 1), j)
                    .expect("Couldn't write! from 'start'. Error_fn: start()_ui.rs_L:271");
            }
            if i == 1 || i % 5 == 1 {
                let j = j.parse::<i32>()
                    .expect("Unable to parse into i32. Error_fn: ui_net()_ui.rs_L:500");
                let j = convert(j / 1024);
                write!(self.stdout, "{}{}", Goto(col_ii, row), j)
                    .expect("Couldn't write! from 'start'. Error_fn: start()_ui.rs_L:271");
            }
            if i == 2 || i % 5 == 2 {
                let j = j.parse::<i32>()
                    .expect("Unable to parse into i32. Error_fn: ui_net()_ui.rs_L:500");
                let j = convert(j);
               write!(self.stdout, "{}{}", Goto(col_iii, row), j)
                .expect("Couldn't write! from 'start'. Error_fn: start()_ui.rs_L:271");
            }
            if i == 3 || i % 5 == 3 {
                let j = j.parse::<i32>()
                    .expect("Unable to parse into i32. Error_fn: ui_net()_ui.rs_L:500");
                let j = convert(j /1024);
              write!(self.stdout, "{}{}", Goto(col_iv, row), j)
              .expect("Couldn't write! from 'start'. Error_fn: start()_ui.rs_L:271");
            }
            if i == 4 || i % 5 == 4 {
                let j = j.parse::<i32>()
                    .expect("Unable to parse into i32. Error_fn: ui_net()_ui.rs_L:500");
                let j = convert(j);
             write!(self.stdout, "{}{}", Goto(col_v, row), j);
            }

            if i % 5 == 0 {
                row += 1;
            }
        }
    }

    pub fn ui(&mut self) {
        let mut position = Position::new();
        let screen_size = Position::screen_size();            // (col, row)-188/39
        let row_p = position.row_p();
        let col_p = position.col_p();
        let pro_head_p = position.process_head_p();

        write!(self.stdout, "{}", Goto(1, row_p))
            .expect("Couldn't write! from 'ui'. Error_fn: ui()_ui.rs_L:531");

        for i in 0..screen_size.x {
            write!(self.stdout, "{}{}─",color::Fg(color::LightRed),color::Bg(color::Black) )
                .expect("Couldn't write! from 'ui'. Error_fn: ui()_ui.rs_L:551");
        }
        let sl = format!("{}│", color::Fg(color::LightRed));
        write!(self.stdout, "{}", Goto(col_p, 1))
            .expect("Couldn't write! from 'ui'. Error_fn: ui()_ui.rs_L:555");

        for i in 0..screen_size.y {
            write!(self.stdout, "{}{}", Goto(col_p,i),sl)
                .expect("Couldn't write! from 'ui'. Error_fn: ui()_ui.rs_L:559");
        }

        let pid_p = 1;
        let state_p = pid_p + position.pid_p();
        let program_p = state_p + position.state_p();
        let command_p = program_p + position.program_p();
        let user_p = command_p + position.command_p();
        let cpu_p = user_p + position.user_p();
        let mem_p = cpu_p + position.cpu_p();

        // %% pid[6.5%]--state[2.0%]--program[14.3%]--command[53%]--user[13.4%]--cpu[5.4%]--mem[5.4%]
        write!(self.stdout, "{}pid{}S{}program{}command{}user{}cpu{}mem",
            Goto(pid_p,row_p + 1),
            Goto(state_p, row_p + 1),
            Goto(program_p, row_p + 1),
            Goto(command_p, row_p + 1),
            Goto(user_p, row_p + 1),
            Goto(cpu_p, row_p + 1),
            Goto(mem_p, row_p + 1)
        ).expect("Couldn't write! from 'ui'. Error_fn: ui()_ui.rs_L:579");

    }

    pub fn ui_proces(&mut self) {
        let mut pid = 0;
        let mut name = "".to_string();
        let mut state = "".to_string();
        let mut command = "".to_string();
        let mut user = "".to_string();
        let mut mem = 0;
        let mut cpu = 0.0;
        let mut threads = 0;
        let mut ppid = 0;

        let mut position = Position::new();
        let size = Position::screen_size();
        let r_point = position.row_p();

        let r_max = size.y - r_point;

        let pid_p = 1;
        let state_p = pid_p + position.pid_p();
        let program_p = state_p + position.state_p();
        let command_p = program_p + position.program_p();
        let user_p = command_p + position.command_p();
        let cpu_p = user_p + position.user_p();
        let mem_p = cpu_p + position.cpu_p();

        let data = &self.data_process;

        let mut ck: Vec<Vec<Data>> = Vec::new();

        for (c,d) in data.iter().enumerate() {
            if self.position > r_max {
                let start = (self.position - r_max) as usize;
                let end = (self.position) as usize;
                if c < start || c > end {
                    continue;
                }
                ck.push(d.clone());
            } else {
                ck.push(d.clone());
            }
        }

        for (i, j) in ck.iter().enumerate() {
            for (x,y) in j.iter().enumerate() {
                match y {
                    Data::I32(k) => {
                        if x == 0 {
                            pid = *k;
                        } else if x == 4 {
                            threads = *k;
                        } else if x == 6 {
                            ppid = *k;
                        } else if x == 7 {
                            mem = *k;
                        }
                    },
                    Data::F32(l) => {
                        cpu = *l;
                    },
                    Data::S(m) => {
                        if x == 1 {
                            state = (*m).to_string();
                            state.truncate(2);
                        } else if x == 2 {
                            name = (*m).to_string();
                            name.truncate(program_p as usize);
                        } else if x == 3 {
                            command = (*m).to_string();
                            command.truncate(command_p as usize);
                        } else if x == 5 {
                            let u = m;
                            user = (*self.uid.get(u).unwrap()).to_string();
                        }
                    },
                }
            }
            let fg_reset = color::Fg(color::Reset);
            let bg_reset = color::Bg(color::Reset);

            let mem = convert(mem);
            let mut  sp = self.position;
            if sp >= r_max {
                sp = r_max;
            }
            if sp == i as u16 {
                self.to_kill = pid.to_string();
                write!(self.stdout, "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                    color::Bg(color::White), color::Fg(color::Black),
                    Goto(pid_p,(r_point + 2) + i as u16), pid,
                    Goto(state_p, (r_point + 2) + i as u16), state,
                    Goto(program_p, (r_point + 2) + i as u16), name,
                    Goto(command_p, (r_point + 2) + i as u16), command,
                    Goto(user_p, (r_point + 2) + i as u16), user,
                    Goto(cpu_p, (r_point + 2) + i as u16), cpu,
                    Goto(mem_p, (r_point + 2) + i as  u16), mem,
                ).expect("Couldn't write! from 'ui_proces'. Error_fn: ui_proces()_ui.rs_L:678");
            } else {
                write!(self.stdout, "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",fg_reset, bg_reset,
                    color::Bg(color::Black), color::Fg(color::White),
                    Goto(pid_p,(r_point + 2) + i as u16), pid,
                    Goto(state_p, (r_point + 2) + i as u16), state,
                    Goto(program_p, (r_point + 2) + i as u16), name,
                    Goto(command_p, (r_point + 2) + i as u16), command,
                    Goto(user_p, (r_point + 2) + i as u16), user,
                    Goto(cpu_p, (r_point + 2) + i as u16), cpu,
                    Goto(mem_p, (r_point + 2) + i as  u16), mem,
                ).expect("Couldn't write! from 'ui_proces'. Error_fn: ui_proces()_ui.rs_L:688");
            }

            if (i as u16) >= r_max {
                break;
            }
        }
    }

    pub fn ui_io(&mut self, io_disk: &Vec<f32>, io_stat: &Vec<f32>) {
        let mut position = Position::new();
        let row = position.row_io();
        let col = position.col_start_ii();
        let col_u = position.utilised_p();
        let col_ab = position.absolute_p();
        let col_t = position.temp_p();

        let data_disk = io_disk;
        let data_io = io_stat;
        let free_disk = &self.free_space;
        let root_unused = 100.0 * free_disk[0] as f32 / data_disk[0];
        let home_unused = 100.0 * free_disk[1] as f32/ data_disk[2];
        let efi_unused = 100.0 * free_disk[2]  as f32 / (data_disk[4] * 1024.0);

        for i in 0..col {
            write!(self.stdout, "{}{}─",color::Fg(color::LightRed), Goto(col + i, row))
                .expect("Couldn't write! from 'start'. Error_fn: start()_ui.rs_L:271");

        }

        let color_b = color::Rgb(255,0,110);
        let color_d = color::Rgb(58,134,255);

        write!(self.stdout, "{0}{1}root{2}{3}%{4}{5}{6}kB",
            color::Fg(color_d),
            Goto(col, row + 1),
            Goto(col_u, row + 1), root_unused,
            Goto(col_ab, row  + 1), data_io[2], data_io[3],
        ).expect("Couldn't write! from 'ui_io'. Error_fn: ui_io()_ui.rs_L:722");

        write!(self.stdout, "{0}{1}{2}{3}{4:.1}GiB",
            color::Fg(color_b),
            Goto(col, row + 2),
            Self::per(root_unused),
            Goto(col_t, row + 2),
            data_disk[0],
        ).expect("Couldn't write! from 'ui_io'. Error_fn: ui_io()_ui.rs_L:730");

        write!(self.stdout, "{0}{1}home{2}{3}%{4}{5}{6}kB",
            color::Fg(color_d),
            Goto(col, row + 3),
            Goto(col_u, row + 3), home_unused,
            Goto(col_ab, row + 3), data_io[4], data_io[5],
        ).expect("Couldn't write! from 'ui_io'. Error_fn: ui_io()_ui.rs_L:737");

        write!(self.stdout, "{0}{1}{2}{3}{4:.1}GiB",
            color::Fg(color_b),
            Goto(col, row + 4), 
            Self::per(home_unused),
            Goto(col_t, row + 4),
            data_disk[2],
        ).expect("Couldn't write! from 'ui_io'. Error_fn: ui_io()_ui.rs_L:745");

        write!(self.stdout, "{0}{1}efi{2}{3}%{4}{5}{6}kB",
            color::Fg(color_d),
            Goto(col, row + 5),
            Goto(col_u, row + 5), efi_unused,
            Goto(col_ab, row + 5), data_io[0], data_io[1],
        ).expect("Couldn't write! from 'ui_io'. Error_fn: ui_io()_ui.rs_L:752");

        write!(self.stdout, "{0}{1}{2}{3}{4:.1}GiB",
            color::Fg(color_b),
            Goto(col, row + 6),
            Self::per(efi_unused),
            Goto(col_t, row + 6),
            data_disk[4]
        ).expect("Couldn't write! from 'ui_io'. Error_fn: ui_io()_ui.rs_L:760");
    }

    pub fn ui_cpu(&mut self, cpu_d: &Vec<f32>, ctemp: &Vec<f32>, cinfo: &Vec<String>, cores: &i32) {
        let mut position = Position::new();
        let screen_size = Position::screen_size();

        //let mut cpu = System::new();
        //cpu.call_s();
        let c_data = cpu_d;
        let c_temp = ctemp;
        let c_speed = cinfo;
        let mut cpu_speed_f:Vec<f32> = Vec::new();
        let mut cpu_avg_speed = 0.0;

        for i in 0..c_speed.len() - 1 {
            let p = c_speed[i].parse::<f32>().unwrap();
            cpu_avg_speed += p;
            cpu_speed_f.push(p);
        }

        let cpu_avg_speed = cpu_avg_speed / (cores * 2) as f32;

        let row_p = position.row_cpu();
        let col_p = position.col_start_ii();
        let col_u = position.utilised_p();
        let col_ab = position.absolute_p();
        let col_t = position.temp_p();

        let len = c_data.len() - 1;
        let mut row_c: u16 = 0;
        for (i,j) in  c_data.iter().enumerate() {
            if i == len {
                write!(self.stdout, "{0}{1}cpu{2}{3:.2}%{4}{5:.1}MHz",
                    //clear::All,
                    color::Fg(color::Rgb(0,127,95)),
                    Goto(col_p, row_p + row_c + i as u16),
                    Goto(col_u, row_p + row_c + i as u16), j,
                    Goto(col_ab, row_p + row_c + i as u16),
                    cpu_avg_speed,
                ).expect("Couldn't write! from 'ui_cpu'. Error_fn: ui_cpu()_ui.rs_L:800");

                write!(self.stdout, "{}{}{}{}{}°C",
                    //clear::All,
                    color::Fg(color::Rgb(43,147,72)),
                    Goto(col_p, row_p + row_c + 1 + i as u16),
                    Self::per(*j),
                    Goto(col_t, row_p + row_c + 1 +i as u16),
                    c_temp[0]
                ).expect("Couldn't write! from 'ui_cpu'. Error_fn: ui_cpu()_ui.rs_L:809");

                row_c += 1;
            } else {
                write!(self.stdout, "{0}{1}cpu{2}{3}{4:.2}%{5}{6:.1}MHz",
                    //clear::All,
                    color::Fg(color::Rgb(56,176,0)),
                    Goto(col_p, row_p + row_c + i as u16), i,
                    Goto(col_u, row_p + row_c + i as u16), j,
                    Goto(col_ab, row_p + row_c + i as u16),
                    cpu_speed_f[i],
                ).expect("Couldn't write! from 'ui_cpu'. Error_fn: ui_cpu()_ui.rs_L:820");

                write!(self.stdout, "{}{}{}{}{}°C",
                    //clear::All,
                    color::Fg(color::Rgb(112,224,0)),
                    Goto(col_p, row_p + row_c + 1 + i as u16),
                    Self::per(*j),
                    Goto(col_t, row_p + row_c + 1 + i as u16),
                    c_temp[(((1.0 + (i as f32)) / 2.0).round()) as usize],
                ).expect("Couldn't write! from 'ui_cpu'. Error_fn: ui_cpu()_ui.rs_L:722");

                row_c += 1;
            }
        }
        //self.stdout.flush().unwrap();
    }

    /*
    fn process_view(&mut self, dir: Direction) {
        let mut processes = all_process();
        let mut point = Position::screen_size();

        let mut start_v = 0;
        let mut end_v: usize = point.y as usize - 12;
        let d = end_v - start_v;

        match dir {
            Direction::Up => {
                if start_v <= 0 {
                    start_v = 0;
                    end_v = point.y as usize - 12;
                } else {
                    start_v -= 1;
                    end_v -= 1;
                }
            },
            Direction::Down => {
                if end_v >= processes.len() + 1 {
                    end_v = processes.len() + 1;
                    start_v = end_v - d;
                } else {
                    start_v += 1;
                    end_v += 1;
                }
            },
            _ => (),
        }
        self.data_process = processes.drain(start_v..end_v).collect();
    }
    */
}

/// BUG -----
/// * Implement return type in string
///   with 'GiB' or 'MiB' suffix
// converting kb into MiB or GiB
fn convert(kb: i32) -> String {
    if kb >= 1024 && kb <= 1048575 {
        return format!("{0:.1}MB", kb as f32 / 1024.0)
    } else if kb >= 1048576 && kb <= 1073741824 {
        return  format!("{0:.1}GB", kb as f32 / 1048576.0)
    } else {
        return format!("{}kb", kb)
    }
}


