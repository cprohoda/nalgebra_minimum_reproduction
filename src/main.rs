use nalgebra::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Blah {
    A,
    B,
    C,
}

fn main() {
    let test = Vector3::new(Blah::A, Blah::B, Blah::C);
    
    println!("{:?}", test)
}
