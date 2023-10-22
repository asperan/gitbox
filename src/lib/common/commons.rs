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

