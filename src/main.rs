
fn main() {

    let number = 3;

    if number < 5 {
        println!("condition was true")
    }else {
        println!("condition was false")
    }

    // Using if in a let Statement

    let condition = true;
    let number = if condition { 5 } else { 6 }; // This means the values that have the potential to be results from each arm of the if must be the same type;

    println!("The value of number is: {}", number);


    // Repetition with Loops

    // Repeating Code with loop

    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    /*loop {
        println!("again!");
    }*/

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);



    // Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


   // Conditional Loops with while

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // Looping Through a Collection with for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len(){
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
