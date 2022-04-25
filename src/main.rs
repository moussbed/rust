
fn main() {
    // Statements and Expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.

    // Expression
    let y = {
        let x = 3;
        x + 1
    };
    /*
    is a block that, in this case, evaluates to 4.
    That value gets bound to y as part of the let statement.
    Note that the x + 1 line doesn’t have a semicolon at the end, unlike most of the lines you’ve seen so far.
    Expressions do not include ending semicolons.
    If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    Keep this in mind as you explore function return values and expressions next.
    */

    println!("The value of y is: {}", y);


    // Call of a function
    let x = five();
    println!("The value of x is: {}", x);

    let t = plus_one(5);
    println!("The value of t is: {}", t);
}

// Functions with Return Values
/*
   Functions can return values to the code that calls them.
   We don’t name return values, but we must declare their type after an arrow (->).
   In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
   You can return early from a function by using the return keyword and specifying a value,
   but most functions return the last expression implicitly.
   Here’s an example of a function that returns a value:
 */
/*
   There are no function calls, macros,
   or even let statements in the five function, just the number 5 by itself.
 */
fn five() -> i32 {
    5
}

/*
 But if we place a semicolon at the end of the line containing x + 1,
 changing it from an expression to a statement, we’ll get an error.
 */
fn plus_one(x: i32) -> i32 {
    x + 1
}
