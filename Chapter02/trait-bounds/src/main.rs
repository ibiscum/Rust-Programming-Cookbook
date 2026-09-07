use std::fmt::Debug;

///
/// An interface that can be used for quick and easy logging
/// 
pub trait Loggable: Debug + Sized {
    fn log(self) {
        println!("{:?}", &self)
    }
}
///
/// A simple print function for printing debug formatted variables
/// 
fn log_debug<T: Debug>(t: T) {
    println!("{:?}", t);
}

#[derive(Debug)]
struct ArbitraryType {
    v: Vec<i32>
}

impl ArbitraryType {
    pub fn new() -> ArbitraryType {
        ArbitraryType {
            v: vec![1,2,3,4]
        }
    }

    pub fn values(&self) -> &[i32] {
        &self.v
    }
}
impl Loggable for ArbitraryType {}

#[derive(Debug)]
struct AnotherType(usize);

impl AnotherType {
    pub fn value(&self) -> usize {
        self.0
    }
}

fn main() {
    let a = ArbitraryType::new();
    println!("v = {:?}", a.values());
    a.log();
    let b = AnotherType(2);
    println!("usize = {}", b.value());
    log_debug(b);
}
