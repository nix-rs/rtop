use std::io::{self, Read, stdout, Write, Stdin};

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

use crate::ui::Ui;

use crate::data::{
    users,
    cal_disk_used,
};


fn main() -> io::Result<()> {

    let stdin = async_stdin();
    let mut stdout = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
    write!(stdout, "{}", termion::cursor::Hide);

    let uid = users();
    let free_space = cal_disk_used();
    let mut uii = Ui::new(stdout, stdin, uid, free_space);
    uii.start();

    Ok(())
}
