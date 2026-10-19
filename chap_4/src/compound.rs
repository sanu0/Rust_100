use std::io;

pub fn tupli(){
    println!("Tuple data type....");
    let tup1 : (i32, f64, u8) = (500, 6.4, 1);

    println!("First value: {}", tup1.0);
    println!("Second value: {}", tup1.1);
    println!("Third value: {}", tup1.2);

    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup2;

    println!("The value of y is: {y}");

    let five_hundred = tup1.0;
    println!("First element is : {}",five_hundred);
}

pub fn arrayf(){
    let a = [1,2,3,4,5]; //Note the in the array elements are of the same type.
    println!("---------inside the arrayf() function-----------------");
    println!("first element: {} , second element : {} ", a[0], a[1]);

    let b: [i32; 5] = [1, 2, 3, 4, 5]; //another way to initialize an array

    let c = [3; 5]; // [3,3,3,3,3] [val, size]
}

pub fn arrayacces(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}


// The program resulted in a runtime error at the point of using an invalid value in the indexing operation. 
// The program exited with an error message and didn’t execute the final println! statement. 
// When you attempt to access an element using indexing, 
// Rust will check that the index you’ve specified is less than the array length. If the index is 
// greater than or equal to the length, Rust will panic. This check has to happen at runtime, 
// especially in this case, because the compiler can’t possibly know what value a user will enter when 
// they run the code later.

// This is an example of Rust’s memory safety principles in action. In many low-level languages, 
// this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. 
// Rust protects you against this kind of error by immediately exiting instead of allowing the 
// memory access and continuing.

