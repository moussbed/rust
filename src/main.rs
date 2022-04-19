
fn main() {
    let a = 5;
    println!("The value of a is {}",a);
   // a = 10; // cannot assign twice to immutable variable

    let mut x = 7;
    println!("The value of x is {}",x);
    x=3;
    println!("The value of x is {}",x);


    // Constants
    const  THREE_HOURS_IN_SECONDS: u32= 60 * 60 * 300;
    println!("The constant value is  {}", THREE_HOURS_IN_SECONDS);


    // Shadowing

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    /*
    The other difference between mut and shadowing is that because we’re effectively creating a new variable
    when we use the let keyword again, we can change the type of the value but reuse the same name.
    The first spaces variable is a string type and the second spaces variable is a number type.
    Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num;
    instead, we can reuse the simpler spaces name.
    */

    let spaces = "   ";
    let spaces = spaces.len();
    println!("We have {} spaces",spaces);

    /*
    However, if we try to use mut for this, as shown here, we’ll get a compile-time error:
    */

    let mut space = "    ";
    space = space.len(); // expected `&str`, found `usize`

}
