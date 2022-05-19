
pub fn calculator_1() {
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

    else if operator == "*".to_string(){
        println!("{} * {} = {}",n1,n2,n1*n2);
    }

    else if operator == "/".to_string(){
        println!("{} / {} = {}",n1,n2,n1/n2);
    }
}


pub fn calculator_2() {
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

    else if operator == "*".to_string(){
        println!("{} * {} = {}",n1,n2,n1*n2);
    }

    else if operator == "/".to_string(){
        println!("{} / {} = {}",n1,n2,n1/n2);
    }
}

fn main() {
    calculator();
}
