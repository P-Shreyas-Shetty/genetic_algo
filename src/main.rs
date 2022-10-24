mod node_base;
use node_base::cmp::Eq;
use node_base::ops::{Add, Mul};
use node_base::Node;
use node_base::{Type, TypeV, Val};

fn main() {
    let v0 = Val::new(Type::int(0));
    let v1 = Val::new(Type::int(0));
    let v2 = Val::new(Type::int(0));
    let v3 = Val::new(Type::int(0));
    let e0 = Add::new(v0, v1);
    let e1 = Mul::new(e0, v2);
    let e2 = Eq::new(e1, v3);
    let args_list: &[Type] = &[];
    println!("{}\n={}", e2.to_str(0), e2.eval(args_list),);
}
