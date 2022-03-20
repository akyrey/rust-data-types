fn main() {
    println!("Available data types");

    println!("> Scalar types");

    println!(">> Integers - number defines bit used (isize and usize depends on arch)");
    println!("* i8 i16 i32 i64 i128 isize");
    println!("* u8 u16 u32 u64 u128 usize");

    println!(">> Floating point - number defines bit used");
    println!("* f32 f64");

    let x_f64 = 2.0; // f64
    let y_f32: f32 = 3.0;

    println!("Operators");
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // Results in 0
    let floored = 2 / 3;
    // remainder
    let remainder = 43 % 5;

    println!(">> Booleans - 1 byte");
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!(">> Char - 4 bytes");
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = ' ';

    println!("> Compound types");

    println!(">> Tuple - fixed length of elements of different types");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!(">> Array - fixed length of elements of the same types");
    // type of elements; array length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // array initialized with length 5, each element will be 3
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}
