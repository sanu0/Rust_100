
// pub fn immuta(){
//     let x = 5;
//     println!("x = {}",x);
//     x = 6;
//     println!("x = {}",x);
// }

pub fn muta(){
    let mut x = 5;
    println!("x = {}",x);
    x = 6;
    println!("x = {}",x);
}

pub fn constants(){
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}",THREE_HOURS_IN_SECONDS);
}

pub fn shadowing(){
    // /**
    //  * you can declare a new variable with the same name as a previous variable. Rustaceans say that the 
    //  * first variable is shadowed by the second, which means that the second variable is what the compiler 
    //  * will see when you use the name of the variable. In effect, the second variable overshadows the first, 
    //  * taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. 
    //  * We can shadow a variable by using the same variable’s name and repeating the use of the 
    //  * let keyword as follows:
    //  */

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //  This program first binds x to a value of 5. Then, it creates a new variable x by repeating let x =, 
    //  taking the original value and adding 1 so that the value of x is 6. Then, within an inner 
    //  scope created with the curly brackets, the third let statement also shadows x and creates a 
    //  new variable, multiplying the previous value by 2 to give x a value of 12. When that scope is over, 
    //  the inner shadowing ends and x returns to being 6.

}

// The other difference between mut and shadowing is that because we’re effectively creating a new 
// variable when we use the let keyword again, we can change the type of the value but reuse the same name. 
// For example, say our program asks a user to show how many spaces they want between some text by inputting 
// space characters, and then we want to store that input as a number:

//     let spaces = "   ";
//     let spaces = spaces.len();

//However, if we try to use mut for this, as shown here, we’ll get a compile-time error.

// Its real advantage is that you can reuse a variable name to represent successive transformations of a value, 
// even when those transformations change its type or make the value immutable again.


// The four useful applications of shadowing

// A. Type transformation — the obvious use case
// B. Reusing a name across transformation stages
// C. Re-establishing immutability after construction
// D. Narrowing a value's scope and lifetime

pub fn shadowing_narrow(){
    let buffer = vec![1u8; 4096];

    // Process the buffer and produce a compact result.
    let buffer = buffer.iter().filter(|&&b| b != 0).count();

    // The original vector is no longer accessible by this name.
    println!("{buffer}");
}