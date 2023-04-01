extern crate uu;
use uu::*;

fn main() {
    println!("Hello, uu!");
    let sch1 = Schema::default();
    let _inst1 = Inst::new(sch1.clone());
    let sch2 = Schema::default();
    let _inst2 = Inst::new(sch2.clone());
    let _system = System::new(sch1, sch2);
}
