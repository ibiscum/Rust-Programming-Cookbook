use rand::prelude::*;

pub fn monte_carlo_pi(iterations: usize) -> f32 {
    let mut inside_circle = 0; 
    for _ in 0..iterations {

        // generate two random coordinates between 0 and 1
        let x: f32 = random::<f32>();
        let y: f32 = random::<f32>();
        
        // calculate the circular distance from 0, 0
        if x.powi(2) + y.powi(2) <= 1_f32 {
            // if it's within the circle, increase the count
            inside_circle += 1;
        }
    }
    // return the ratio of 4 times the hits to the total iterations
    (4_f32 * inside_circle as f32) / iterations as f32
}

#[cfg(test)]
mod tests {
    // import the parent crate's functions

    use super::*;

    fn average_estimate(iterations: usize, runs: usize) -> f32 {
        let sum: f32 = (0..runs).map(|_| monte_carlo_pi(iterations)).sum();
        sum / runs as f32
    }

    fn is_reasonably_pi(pi: f32) -> bool {
        pi >= 3_f32 && pi <= 4.5_f32
    }

    #[test]
    fn test_monte_carlo_pi_1()  {
        let pi = monte_carlo_pi(1);
        assert!(pi == 0_f32 || pi == 4_f32);
    }

    #[test]
    fn test_monte_carlo_pi_500() {
        let pi = monte_carlo_pi(500);
        assert!(is_reasonably_pi(pi));
    }
    
    #[test]
    fn test_monte_carlo_pi_1000() {
        let pi = monte_carlo_pi(1000);
        assert!(is_reasonably_pi(pi));
    }

    #[test]
    fn test_monte_carlo_pi_5000() {
        let pi = monte_carlo_pi(5000);
        assert!(is_reasonably_pi(pi));
    }

    #[test]
    fn test_monte_carlo_pi_zero_iterations() {
        // Regression: zero iterations produce NaN because of 0/0.
        let pi = monte_carlo_pi(0);
        assert!(pi.is_nan());
    }

    #[test]
    fn test_monte_carlo_pi_convergence() {
        // Regression: averaged estimates should be stable and improve with more iterations.
        let pi_100 = average_estimate(100, 20);
        let pi_1000 = average_estimate(1000, 20);
        let pi_5000 = average_estimate(5000, 20);
        assert!(is_reasonably_pi(pi_100));
        assert!(is_reasonably_pi(pi_1000));
        assert!(is_reasonably_pi(pi_5000));

        // The 5000-iteration estimate should be at least as close to true pi as the 100-iteration one
        // most of the time. Use a loose bound because the algorithm is randomized.
        let true_pi = std::f32::consts::PI;
        let err_100 = (pi_100 - true_pi).abs();
        let err_5000 = (pi_5000 - true_pi).abs();
        assert!(
            err_5000 <= err_100 + 0.2,
            "longer run should not be drastically worse (err_5000={}, err_100={})",
            err_5000,
            err_100
        );
    }
}
