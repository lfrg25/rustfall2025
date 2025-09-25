// Problem 1
fn concat_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

// Problem 2
fn clone_and_modify(s: &String) -> String {
    let mut copy = s.clone();
    copy.push_str("World!");
    copy
}

// Problem 3
fn sum(total: &mut i32, low: i32, high: i32) {
    let mut temp = 0;
    for i in low..=high {
        temp += i;
    }
    *total = temp;
}

fn main() {
    // test problem 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result);

    // test problem 2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s);
    println!("Modified: {}", modified);

    // test problem 3
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total: {}", total);
}
