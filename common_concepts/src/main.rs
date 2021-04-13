fn main() {
    variables_mutability_example();
    shadowing_example();
    scalar_datatype_examples();
    complex_datatype_examples();
    statement_expression_example();
    function_return_value_example();
}


fn variables_mutability_example(){
    let mut x = 1;
    println!("That original value of x is {}", x);
    x = 2;
    println!("That modified value of x is {}", x);
}

fn shadowing_example(){
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn scalar_datatype_examples(){

     // addition
     let sum = 5 + 10;
     println!("Sum is {}", sum);

     // subtraction
     let difference = 95.5 - 4.3;
     println!("difference is {}", difference);

     // multiplication
     let product = 4 * 30;
     println!("product is {}", product);  
     // division
     let quotient = 56.7 / 32.2;
     println!("quotient is {}", quotient);  
     // remainder
     let remainder = 43 % 5;
     println!("remainder is {}", remainder);  
}

fn complex_datatype_examples(){
    //tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    println!("The value of first is: {}", first);
    let second = a[1];
    println!("The value of second is: {}", second);
}

fn statement_expression_example(){
    let x = 5;
    println!("The value of x is: {}", x);

    let y = {     //statement
        let x = 3; 
        x + 1    //expression
    };

    println!("The value of y is: {}", y);
}

fn function_return_value_example(){
    let frv = plus_one(5);

    println!("The value of frv is: {}", frv);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}