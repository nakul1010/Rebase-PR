
pub fn calculator() {
    println!("Enter nos plus operator ");
    let n1 = 100;
    let n2 = 200;
    let operator = String::from("+");

    if operator == "+".to_string(){
        println!("{} + {} = {}",n1,n2,n1+n2);
    }
    
    else if operator == "-".to_string(){
        println!("{} - {} = {}",n1,n2,n1-n2);
    }

    else if operator == "-".to_string(){
        println!("{} - {} = {}",n1,n2,n1-n2);
    }

    else if operator == "-".to_string(){
        println!("{} - {} = {}",n1,n2,n1-n2);
    }

    else if operator == "/".to_string(){
        println!("{} / {} = {}",n1,n2,n1/n2);
    }
}

pub fn hello_world() {
    println!("Hello World");
}

<<<<<<< HEAD
pub fn addition_of_two_nos(n1:i32,n2:i32)
{
    println!("{} + {} = {}",n1,n2,n1+n2);
}


pub fn subtract_of_two_nos(n1:i32,n2:i32)
{
    println!("{} - {} = {}",n1,n2,n1-n2);
}

pub fn add_of_three_nos(n1:i32,n2:i32,n3:i32)
{
    println!("{} + {} + {} = {}",n1,n2,n3,n1+n2+n3);
}

=======
>>>>>>> Add multiplication and division
pub fn multiply_of_two_nos(n1:i32,n2:i32)
{
    println!("{} * {} = {}",n1,n2,n1*n2);
}


pub fn divide_of_two_nos(n1:i32,n2:i32)
{
    println!("{} / {} = {}",n1,n2,n1/n2);
}
fn main() {
    calculator();
}
