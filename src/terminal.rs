use crossterm::terminal::size;

pub fn get_size() -> (u16, u16) {
    let s = size().unwrap();

    s
}
