pub mod enums;
pub mod generics;
pub mod go_model;
fn main() {
    println!("*****GO_MODEL*****");
    go_model::run();
    println!("**********");
    println!("*****Generics*****");
    generics::run();
    println!("*****ENUMS*****");
    enums::run();
    println!("**********");
}
