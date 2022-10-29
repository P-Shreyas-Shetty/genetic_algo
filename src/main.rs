mod node;
use node::base::{BuilderParams, Type, TypeV, Val};
use node::cmp_nodes::Eq;
use node::op_nodes::{Add, Mul};

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
    let mut params = BuilderParams::new().max_depth(5);
    let table = node::btables::FloatFnTable::new().table;
    let root = table
        .get_rand_node(0, TypeV::Float, &mut params)
        .build_random_node(
            &table,
            &[TypeV::Float, TypeV::Float],
            TypeV::Float,
            0,
            &mut params,
        );
    println!(
        "Random tree\n{}\nResult={}",
        root.to_str(0),
        root.eval(&[Type::Float(0.0), Type::Float(1.0)])
    );
}
