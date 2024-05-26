use std::io;

use crossterm::ExecutableCommand;

mod page1;

let room_list: [&Str,2] = ["Room 1", "Room 2"]

fn main() -> io::Result<()> {
    page1::run_page_1()
}



