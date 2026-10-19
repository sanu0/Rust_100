pub fn first_func() {
    println!("first_func()");

    another_function();
    parameter_function(5);
    let ret = return_function(5);
    println!("return value from the return_function is {}" , ret);
}

pub fn another_function() {
    println!("Another function.");
}

pub fn parameter_function(x : i32){ //similarly, you can alsoi pass multiple parameters
    println!("parameter passed : {}", x);
}

pub fn return_function(x: i32) -> i32{
    println!("passed value on this function : {}" , x);
    let val = x * 2;
    return val;
    //val
}