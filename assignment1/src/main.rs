//const FREEZE: i32 = 32;
use std::io;

const FREEZE: i32 = 32;

fn assignment1(){
    println!("ASSIGNMENT 1 OUTPUTS");
    let mut temp_f: i32 = FREEZE;
    let c0 = fahrenheit_to_celsius(temp_f as f64);
    println!("{}", c0);
    for step in 1..=5 {
        let f_next = (temp_f + step) as f64;
        println!("{}", fahrenheit_to_celsius(f_next));
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
} 

fn main(){
    assignment1();
    assignment2();
    assignment3(15);
}

fn is_even(n: i32) -> bool{
    n % 2 == 0
}

fn assignment2(){
    let arr: [i32; 10] = [3,5,15,30,2,8,9,60,54,67];

    for element in arr {
        if element % 3 == 0 && element % 5 == 0 {
            println!("FizzBuzz");
        } else if element % 3 == 0 {
            println!("Fizz");
        } else if element % 5 == 0 {
            println!("Buzz");
        } else {
            let parity = if is_even(element) { "even" } else { "odd" };
            println!("{parity}");
        }
    }

    let arr2: [i32; 10] = [3,5,15,30,2,8,9,60,54,67];
    let mut i = 0;
    let mut sum = 0;
    while i < arr2.len() {
        sum += arr2[i];
        i += 1;
    }
    println!("{}", sum);

    let mut largest = arr2[0];
    for &n in arr2.iter() {
        if n > largest {
            largest = n;
        }
    }
    println!("{}", largest);
}

fn assignment3(secret: i32) {
    let mut guess: i32 = 10;
    let mut attempts: i32 = 0;
    loop {
        attempts += 1;
        let res = check_guess(guess, secret);
        if res == 0 {
            println!("correct");
            break;
        } else if res == 1 {
            println!("too high");
            guess -= 1;
        } else {
            println!("too low");
            guess += 1;
        }
    }
    println!("{}", attempts);
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret { 0 } else if guess > secret { 1 } else { -1 }
}