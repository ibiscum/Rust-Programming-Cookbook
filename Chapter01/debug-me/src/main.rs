struct MyStruct {
    prop: usize,
}

struct Point(f32, f32);

#[allow(unused_variables)]
fn main() {
    let a = 42;
    let b = vec![0, 0, 0, 100];
    let c = [1, 2, 3, 4, 5];
    let d = 0x5ff;
    let e = MyStruct { prop: 10 };
    let p = Point(3.14, 3.14);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_fields_initialized_correctly() {
        let s = MyStruct { prop: 10 };
        assert_eq!(s.prop, 10);
    }

    #[test]
    fn tuple_struct_holds_values() {
        let p = Point(3.14, 3.14);
        assert_eq!(p.0, 3.14);
        assert_eq!(p.1, 3.14);
    }

    #[test]
    fn literals_regression() {
        // Regression test: ensure the demo literals have the expected values.
        assert_eq!(0x5ff, 1535);
        assert_eq!(42_i32 as i32, 42);
        assert_eq!(vec![0, 0, 0, 100], [0, 0, 0, 100]);
        assert_eq!([1, 2, 3, 4, 5], [1, 2, 3, 4, 5]);
    }
}
