//const FREEZE: i32 = 32;

fn assignment1(){
    let array: [f64; 5] = [33.0, 34.0, 35.0, 36.0, 37.0];
    println!("ASSIGNMENT 1 OUTPUTS");
    for i in array.iter(){
        let output: f64 = fahrenheit_to_celsius(*i);
        println!("{}", output);
    }


}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let celsius: f64 = (f - 32.0) * 5.0/9.0;
    return celsius;
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let fahrenheit: f64 = (c * 9.0/5.0) + 32.0;
    return fahrenheit;
} 

fn main(){
    assignment1()
}

//take in a integer as a parameter for both
// then convert that integer using the algo for f -> celsius:
    // 	(°F − 32) × 5/9 = C

//then do the same for c -> f:
    //(C × 9/5) + 32 = 32°F


// be sure to call your functions