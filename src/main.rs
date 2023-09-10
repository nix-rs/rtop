use std::io::{self, stdout, Write};

pub mod data;
mod ui;

use termion::{
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
    async_stdin,
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
