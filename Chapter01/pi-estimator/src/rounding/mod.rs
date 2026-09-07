
pub fn round(nr: f32, places: usize) -> f32 {
    let multiplier = 10_f32.powi(places as i32);
    let shifted = nr * multiplier;

    if shifted.is_sign_negative() {
        (shifted - 0.5).ceil() / multiplier
    } else {
        (shifted + 0.5).floor() / multiplier
    }
}


#[cfg(test)]
mod tests {
    use super::round;

    #[test]
    fn round_positive()  {
       assert_eq!(round(3.123456, 2), 3.12);
       assert_eq!(round(3.123456, 4), 3.1235);
       assert_eq!(round(3.999999, 2), 4.0);
       assert_eq!(round(3.0, 2), 3.0);
       assert_eq!(round(9.99999, 2), 10.0); 
       assert_eq!(round(0_f32, 2), 0_f32);
    }

    #[test]
    fn round_negative()  {
       assert_eq!(round(-3.123456, 2), -3.12);
       assert_eq!(round(-3.123456, 4), -3.1235);
       assert_eq!(round(-3.999999, 2), -4.0);
       assert_eq!(round(-3.0, 2), -3.0);
       assert_eq!(round(-9.99999, 2), -10.0);
    }

    #[test]
    fn round_zero_places() {
        // Regression: rounding to zero decimal places should behave like rounding to integer.
        assert_eq!(round(3.7, 0), 4.0);
        assert_eq!(round(3.2, 0), 3.0);
        assert_eq!(round(-3.7, 0), -4.0);
    }

    #[test]
    fn round_no_fractional_change() {
        // Regression: rounding a value that already has fewer digits should be idempotent.
        assert_eq!(round(2.5, 5), 2.5);
        assert_eq!(round(2.5, 1), 2.5);
    }

    #[test]
    fn round_half_away_from_zero() {
        // Regression: tie-breaking should be consistent for positive and negative numbers.
        assert_eq!(round(2.5, 0), 3.0);
        assert_eq!(round(-2.5, 0), -3.0);
        assert_eq!(round(-1.25, 1), -1.3);
    }

    #[test]
    fn round_small_magnitude_values() {
        // Regression: tiny values should round to 0 when precision is lower than their magnitude.
        assert_eq!(round(0.00001, 4), 0.0);
        assert_eq!(round(-0.00001, 4), -0.0);
    }
}
