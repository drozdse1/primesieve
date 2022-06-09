use ps::SingleThreadSieve;

pub mod ps;

fn main() {
    let mut ps = SingleThreadSieve::init();
    
    ps.calc();

    println!("Hello, world!");
}
