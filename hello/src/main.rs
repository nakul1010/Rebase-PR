pub fn hello_nakul() {
    println!("Hello, Nakul!");
}

pub fn hello_yash() {
    println!("Hello, Yash!");
}

pub fn hello_raj() {
    println!("Hello, Raj!");
}

pub fn hello_alice() {
    println!("Hello, Alice!");
}

pub fn hello_shounak() {
    println!("Hello, Shounak!");
    for i in 0..100 {
        println!("1");
    }
    println!("Hello, Shounak!");
    for i in 0..100 {
        println!("1");
    }
}

pub fn add_function(no1: i32, no2: i32) {
    println!("Addition : {}", no1 + no2);
}
pub fn sub_function(no1: i32, no2: i32) {
    println!("Sub : {}", no1 - no2);
}
pub fn mul_function(no1: i32, no2: i32) {
    println!("Sub : {}", no1 * no2);
}
pub fn after_rebase() {
    println!("After Rebase");
}

fn main() {
    hello_nakul();
    add_function(3 as i32, 3 as i32);
    sub_function(1 as i32, 3 as i32);
}
