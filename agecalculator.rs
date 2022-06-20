
use std::io::{stdin, stdout, Write};

fn main () {

    fn read(input: &mut String) {
        stdout().flush()
            .expect("failed to flush");
        stdin().read_line(input)
            .expect("failed to read");
    }


    const YEAR: f32 = 365.0; //Numbers of days in a year

    let emoji = '\u{1F44B}';

    println!("Welcome to Mychcoder's Age Calculator {}",emoji);
    println!("---------------------------------");


    println!("Please Input your Name?");

    let mut person = String::new();

    read(&mut person);

    print!("Welcome {}",person);

        
    let mut age = String::new();
        
    print!("How Old Are You?: ");

    read(&mut age);

    let age: f32 = age.trim().parse().unwrap();

    let daily_rest = 8.0 * YEAR;

    let days_spent_yearly = daily_rest/24.0;
        
    let years_spent_so_far = days_spent_yearly*age/YEAR;

    println!("You have spent {} years in your lifetime sleeping.",years_spent_so_far);
        
    println!("Your real age is {} years.", age-years_spent_so_far);

}
