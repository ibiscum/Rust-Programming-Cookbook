// declare the module by its file name
mod rounding;

// Rust will also accept if you implement it right away
mod printer {
    // import a function from an external crate (no more extern declaration required!)
    use rust_pilib::monte_carlo_pi;

    // crates present in the parent can be imported using the crate prefix
    use crate::rounding::round;

    pub fn pretty_print_pi_approx(iterations: usize) {
        let pi = monte_carlo_pi(iterations);
        let places: usize = 2;
    
        println!("Pi is ~ {} and rounded to {} places {}", pi, places, round(pi, places));
    }
}

// import from the module above
use printer::pretty_print_pi_approx;


fn main() {
    pretty_print_pi_approx(100_000);
}

#[cfg(test)]
mod tests {
    use super::pretty_print_pi_approx;

    #[test]
    fn pretty_print_smoke_test() {
        // Regression: pretty_print_pi_approx should run without panicking.
        pretty_print_pi_approx(100);
    }

    #[test]
    fn pretty_print_zero_iterations() {
        // Regression: zero iterations produce infinity but should not panic.
        pretty_print_pi_approx(0);
    }
}
