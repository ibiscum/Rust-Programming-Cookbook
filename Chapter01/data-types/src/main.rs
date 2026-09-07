use data_types::MyCustomStruct;

fn build_and_sum() -> f32 {
    let value = MyCustomStruct::new(1, 2, 3.0);
    value.sum()
}

fn main() {
    let total = build_and_sum();
    println!("sum = {}", total);
}

#[cfg(test)]
mod tests {
    use super::build_and_sum;

    #[test]
    fn binary_uses_library_type() {
        assert_eq!(build_and_sum(), 6.0);
    }
}
