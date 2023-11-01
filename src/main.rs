mod greetings;
//use greetings::default_greeting;
//use greetings::default_greeting2
//use greetings::*
use greetings::{default_greeting, default_greeting2,english, spanish, french};

fn main() {
    println!("Hello, world!");
    println!("{}" , default_greeting());
    println!("{}" , default_greeting());
    println!("My first greeting is", "{}",  "and the second is " ,"{}");
    english::default_greeting();
    spanish::default_greeting();
    french::default_greeting();
}



