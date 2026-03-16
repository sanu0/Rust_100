pub fn Inte(){
    println!("Data Type : Integer");
    
    /**
     * Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, 
     * where n is the number of bits that variant uses. 
     * So, an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127. 
     * Unsigned variants can store numbers from 0 to 2n − 1, 
     * so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.
     * 
     */

    let sign_int_1 : i8 = 5;
    let sign_int_2 : i16 = 5;
    let sign_int_3 : i32 = 5;
    let sign_int_4 : i64 = 5;
    let sign_int_5 : i128 = 5;

    let unsign_int_1 : u8 = 5;
    let unsign_int_2 : u16 = 5;
    let unsign_int_3 : u32 = 5;
    let unsign_int_4 : u64 = 5;
    let unsign_int_5 : u128 = 5;

    println!("Value: {}, Type: {}", sign_int_1, std::any::type_name_of_val(&sign_int_1));
    println!("Value: {}, Type: {}", sign_int_2, std::any::type_name_of_val(&sign_int_2));
    println!("Value: {}, Type: {}", sign_int_3, std::any::type_name_of_val(&sign_int_3));
    println!("Value: {}, Type: {}", sign_int_4, std::any::type_name_of_val(&sign_int_4));
    println!("Value: {}, Type: {}", sign_int_5, std::any::type_name_of_val(&sign_int_5));

    println!("Value: {}, Type: {}", unsign_int_1, std::any::type_name_of_val(&unsign_int_1));
    println!("Value: {}, Type: {}", unsign_int_2, std::any::type_name_of_val(&unsign_int_2));
    println!("Value: {}, Type: {}", unsign_int_3, std::any::type_name_of_val(&unsign_int_3));
    println!("Value: {}, Type: {}", unsign_int_4, std::any::type_name_of_val(&unsign_int_4));
    println!("Value: {}, Type: {}", unsign_int_5, std::any::type_name_of_val(&unsign_int_5));
}

pub fn Floati(){
    let x : f32 = 3.22;
    let y : f64 = 89.67;

    let z = 67.89; //f64 by default;

    println!("Value: {}, Type: {}", x, std::any::type_name_of_val(&x));
    println!("Value: {}, Type: {}", y, std::any::type_name_of_val(&y));
    println!("Value: {}, Type: {}", z, std::any::type_name_of_val(&z));
}

pub fn Numeri() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    
    println!("Value: {}, Type: {}", sum, std::any::type_name_of_val(&sum));
    println!("Value: {}, Type: {}", difference, std::any::type_name_of_val(&difference));
    println!("Value: {}, Type: {}", product, std::any::type_name_of_val(&product));
    println!("Value: {}, Type: {}", quotient, std::any::type_name_of_val(&quotient));
    println!("Value: {}, Type: {}", truncated, std::any::type_name_of_val(&truncated));
    println!("Value: {}, Type: {}", remainder, std::any::type_name_of_val(&remainder));

}

pub fn Boole() {
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("Value: {}, Type: {}", t, std::any::type_name_of_val(&t));
    println!("Value: {}, Type: {}", f, std::any::type_name_of_val(&f));   
}

pub fn Chare() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Value: {}, Type: {}", heart_eyed_cat, std::any::type_name_of_val(&heart_eyed_cat));
}


