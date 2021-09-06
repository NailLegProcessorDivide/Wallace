mod numeric_utils;
use numeric_utils::Denotation;

fn main() {
    let mut a = Denotation::new_i128(6);
    let b = Denotation::new_str("1.2");
    let c = a + b;
    a += c;
    println!("Hello, world! a: {}, b: {}, c: {}", a.get_f64(), b.get_i64(), c.get_i64());
}
