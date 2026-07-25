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
    4
}