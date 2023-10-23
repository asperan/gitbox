use std::{io::Write, path::Path};

pub fn print_error_and_exit(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1)
}

pub fn print_cli_error_message_and_exit(stderr: &Vec<u8>, what: &str) -> ! {
    print_error_and_exit(
        &format!(
            "Failed to {}: {}",
            what,
            std::str::from_utf8(stderr).unwrap_or("failed to parse git error, unknown error")
        )
    )
}

pub fn read_lines(path: &std::path::Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let read_result = std::fs::read(path);
    match read_result {
        Ok(content) => {
            match std::string::String::from_utf8(content) {
                Ok(s) => Ok(s.split("\n").filter(|s| !s.is_empty()).map(|line| line.to_string()).collect()),
                Err(e) => Err(Box::new(e)),
            }
        },
        Err(e) => Err(Box::new(e)),
    }
}

pub fn append_line(path: &str, line: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::options().append(true).open(path).unwrap();
    write!(f, "\n{}", line)
}

pub fn ensure_dir_exists(path: &str) {
    let dir = Path::new(path);
    if !dir.exists() {
        match std::fs::create_dir_all(dir) {
            Ok(()) => {}
            Err(e) => eprintln!(
            "Failed to create directory {}: {}",
            dir.display(),
            e
        ),
        }
    }
}
