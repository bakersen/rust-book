fn main() {
    //Integers
    let _small_number:usize = 18_446_744_073_709_551_115;

    //floating points
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    //Numerical operations
    // addition
    let sum = 6.0 + 7.0;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = 5.0 / 3.0; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    println!("sum: {sum}, truncated: {truncated} quotient: {quotient}, difference: {difference}, product: {product}");

    //BOOLEANS
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    //Tuples
    let tuple: (u32, String, f32) = (500, String::from("Baker"), 5.0);

    // let (int, string, float) = tuple;
    // println!("{int} {string} {float}");

    println!("{} {} {}", tuple.0, tuple.1, tuple.2);

    //Arrays
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("{}, {}, {}, {}", months[0], months[1], months[2], months[3]);

    let a:[u8; 5] = [0,1,2,3,4];
    println!("{}, {}, {}, {}",a[0],a[1],a[2],a[3]);

}
