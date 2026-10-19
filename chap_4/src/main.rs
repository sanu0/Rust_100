mod scalar;
mod compound;
mod functions;
mod controlflow;

fn main() {
    println!("INSIDE MAIN");

    scalar::Inte();
    scalar::Floati();
    scalar::Numeri();
    scalar::Boole();
    scalar::Chare();
    compound::tupli();
    compound::arrayf();
    compound::arrayacces();
    functions::first_func();
    controlflow::first(6);
    controlflow::conditional_statement();
    controlflow::loopf();
    controlflow::break_outer();
    controlflow::loop_examples();

}
