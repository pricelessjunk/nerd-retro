use std::io;

use crossterm::ExecutableCommand;

mod page1;

fn main() -> io::Result<()> {
    page1::run_page_1()
}
