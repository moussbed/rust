use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    println!("Guess number");
    let secret_number=rand::thread_rng().gen_range(1..101);
    let mut  count=0;
    loop {
        println!("Please input your guess between 1 and 100");
        count=count+1;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32= match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                eprintln!("You must enter number");
                continue
            },
        };
        println!("You guess : {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less=> println!("Too small !"),
            Ordering::Greater=> println!("Too big !"),
            Ordering::Equal=> break
        }
    }
    println!("Attempt number {}", count);


}
