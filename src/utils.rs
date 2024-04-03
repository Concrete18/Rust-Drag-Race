use crossterm::{
    cursor::{position, MoveTo},
    execute,
};
use std::io::{self, Write};

/// Replaces the line at `line_num` with `new_contents` relative to `start_line`
pub fn replace_line(line_num: u16, new_contents: String, start_line: u16) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // Move cursor to line 5 (0-based index)
    execute!(handle, MoveTo(0, start_line + line_num)).unwrap();
    // Clear line and print new content
    write!(handle, "{new_contents}").unwrap();
    handle.flush().unwrap();
}

/// Gets the current line number in the terminal
pub fn get_current_line_number() -> Result<u16, crossterm::ErrorKind> {
    let (_, line) = position()?;
    Ok(line)
}

/// Asks for input after printing a msg
pub fn input() -> String {
    let mut response: String = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    return response.trim().to_string();
}
