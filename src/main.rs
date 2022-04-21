
fn main() {
    /*
     Keep in mind that Rust is a statically typed language,
     which means that it must know the types of all variables at compile time.
     The compiler can usually infer what type we want to use based on the value and how we use it.
     In cases when many types are possible, such as when we converted a String to a numeric type using parse,
     we must add a type annotation
     */

    let guess :u32 = "42".parse().expect("Not a number");
    // let guess = "42".parse().expect("Not a number"); // Compile error: consider giving `guess` a type

    // Scalar Types
    /* A scalar type represents a single value.
      Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    */

    // Floating-Point Types

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2; // Results 1.7608695652173911
    println!("{}",quotient);
    let floored = 2 / 3; // Results in 0
    println!("{}",floored);

   // let floored = 2.5 / 3; // Compilation Error: no implementation for `{float} / {integer}`

    // remainder
    let remainder = 43 % 5;
    println!("{}",remainder);


    // Boolean
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("{}",f);


    // The Character Type

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}",c);
    println!("{}",z);
    println!("{}",heart_eyed_cat);


    // Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    /*
    The Tuple Type
    A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a,b,c) = tup; // destructuring
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);

    let a = tup.0; // We use here indice
    let b = tup.1;
    let c = tup.2;

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);


    /*
     The tuple without any values, (), is a special type that has only one value, also written ().
     The type is called the unit type and the value is called the unit value.
     Expressions implicitly return the unit value if they don’t return any other value.
     */
    let unit_value = ();


    /*
      The Array Type
      Another way to have a collection of multiple values is with an array.
      Unlike a tuple, every element of an array must have the same type.
      Unlike arrays in some other languages, arrays in Rust have a fixed length.
    */

    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // You can also initialize an array to contain the same value for each element by specifying
    // the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

    let same_values = [3;5]; // let same_values = [3, 3, 3, 3, 3];

}
