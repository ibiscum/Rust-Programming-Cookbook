use std::io::{self, Write};
use std::f64;

/// Formats a number with the given precision.
pub fn format_with_precision(value: f64, precision: usize) -> String {
    format!("{value:.precision$}", value = value, precision = precision)
}

/// Formats a number as zero-padded hex with the given width.
pub fn format_padded_hex(value: usize, width: usize) -> String {
    format!("{value:0width$x}", value = value, width = width)
}

/// Formats a number as right-aligned decimal with the given width.
pub fn format_padded_decimal(value: usize, width: usize) -> String {
    format!("{value:>width$}", value = value, width = width)
}

fn main() {
    // Basic printing with arguments
    println!("Let's print some lines:");
    println!();
    println!("Hello, world!");
	println!("{}, {}!", "Hello", "world");
    // No newlines    
    print!("Hello, ");
    println!("world!");
    
    println!("Arguments can be referred to by their position: {0}, {1}! and {1}, {0}! are built from the same arguments", "Hello", "world");

    // More complex arguments
	println!("Furthermore the arguments can be named: \"{greeting}, {object}!\"", greeting = "Hello", object = "World");

    // Number formatting
	println!("Number formatting: Pi is {0:.3} or {0:.0} for short", f64::consts::PI);

    // Padding and hex formatting
    println!("... and there is more: {0:>0width$}={0:>width$}={0:#x}", 1535, width = 5);

    // Writing to a stream directly
    let _ = write!(&mut io::stdout(), "Underneath, it's all writing to a stream...");
    println!();

    // Reading from std::in
    println!("Write something!");
    let mut input = String::new();
    if let Ok(n) =  io::stdin().read_line(&mut input) {
        println!("You wrote: {} ({} bytes) ", input, n);
    }
    else {
        // Printing to std::err
        eprintln!("There was an error :(");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precision_formatting() {
        assert_eq!(format_with_precision(f64::consts::PI, 3), "3.142");
        assert_eq!(format_with_precision(f64::consts::PI, 0), "3");
    }

    #[test]
    fn padded_hex_formatting() {
        assert_eq!(format_padded_hex(1535, 5), "005ff");
    }

    #[test]
    fn padded_decimal_formatting() {
        assert_eq!(format_padded_decimal(1535, 5), " 1535");
    }

    #[test]
    fn precision_regression_zero() {
        // Regression: formatting zero with any precision should stay zero.
        assert_eq!(format_with_precision(0.0, 5), "0.00000");
    }
}

