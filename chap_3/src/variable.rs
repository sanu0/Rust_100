/** 
pub fn immuta(){
    let x = 5;
    println!("x = {}",x);
    x = 6;
    println!("x = {}",x);
}
*/
pub fn muta(){
    let mut x = 5;
    println!("x = {}",x);
    x = 6;
    println!("x = {}",x);
}