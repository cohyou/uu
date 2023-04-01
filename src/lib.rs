mod parser;
mod romania;
mod australia;
mod knuth_bendix;

pub use parser::*;
pub use romania::*;
pub use australia::*;
pub use knuth_bendix::*;

type Prim = String;
type Tp = String;
#[derive(Clone)]
struct Func;
#[derive(Clone)]
struct Rule;

#[derive(Default, Clone)]
pub struct Schema {
    _prims: Vec<Prim>,
    _types: Vec<Tp>,
    _funcs: Vec<Func>,
    _rules: Vec<Rule>,
}

pub struct Inst {
    _schema: Schema,
}

impl Inst {
    pub fn new(sch: Schema) -> Self { Inst { _schema: sch } }
}

pub struct System {
    _dom: Schema,
    _cod: Schema,
}

impl System {
    pub fn new(dom: Schema, cod: Schema) -> Self { System { _dom: dom, _cod: cod} }
}