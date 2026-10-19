use std::io;

pub fn first(number: i32){

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
} 

pub fn conditional_statement(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

pub fn loopf(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

pub fn break_outer(){
    let mut count = 0;
    'counting_up: loop{

        println!("count = {}",count);
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }
    println!("End count = {}",count);
}
// The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop without a 
// label counts down from 10 to 9. The first break that doesn’t 
// specify a label will exit the inner loop only. The break 'counting_up; statement will exit the outer loop

pub fn loop_examples(){

    println!("-----loop examples 1--------");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("-----loop examples 2--------");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("-----loop examples 3--------");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!("-----loop examples 4--------");
   
    for elem in a.iter().rev(){
        println!("{}",elem);
    }
}