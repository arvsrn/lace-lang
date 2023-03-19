use colored::Colorize;

#[derive(PartialEq)]
pub enum ErrorType {
    Warning,
    Fatal,
}

pub fn print(
    message: &str,     // Error message
    lines: &Vec<&str>, // All the lines in the source
    error_line: usize, // Line where the error occured
    error_char: usize,
    error_type: ErrorType,
) {
    if error_line != 0 {
        println!(
            "{}",
            format!("{} | {}", error_line - 1, lines[error_line - 1]).dimmed()
        );
    } else {
        println!(
            "{}",
            format!("{} |", " ".repeat(error_line.to_string().len())).dimmed()
        )
    }

    println!(
        "{}",
        format!("{} {} {}", error_line, "|".dimmed(), lines[error_line])
    );
    println!(
        "{}",
        format!(
            "{} {} {}^ {}",
            " ".repeat(error_line.to_string().len()),
            "|".dimmed(),
            " ".repeat(error_char),
            if error_type == ErrorType::Fatal {
                message.red()
            } else {
                message.yellow()
            }
        )
    );

    if error_line + 1 >= lines.len() {
        println!(
            "{}",
            format!("{} |", " ".repeat(error_line.to_string().len())).dimmed()
        )
    } else {
        println!(
            "{}",
            format!("{} | {}", error_line + 1, lines[error_line + 1]).dimmed()
        );
    }
}
