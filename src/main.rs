mod algorithm;
mod node;

use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;
use rand::Rng;

fn main() {
    let table = FloatFnTable::new().table;
    let params = nb::BuilderParams::new().max_depth(6);
    let mut params0 = nb::BuilderParams::new().max_depth(6);
    let mut popln = ap::Population::new(vec![nb::TypeV::Float, nb::TypeV::Float], nb::TypeV::Float);
    let (train_x, train_y) = {
        let mut train_x = Vec::<Vec<nb::Type>>::new();
        let mut train_y = Vec::<nb::Type>::new();
        for _ in 0..100 {
            let (x, y) = (
                params0.randomizer.gen_range(-100.0f32..=100.0f32),
                params0.randomizer.gen_range(-100.0f32..=100.0f32),
            );
            train_x.push(vec![nb::Type::float(x), nb::Type::float(y)]);
            train_y.push(nb::Type::float(x + y));
        }
        (train_x, train_y)
    };

    popln.set_build_table(table);
    popln.set_params(params);
    let mut top_kid = popln.train(&train_x, &train_y);
    top_kid.calc_err(&train_x, &train_y);

    for (i, p) in popln.p.iter().enumerate() {
        println!(
            ">>>[{i}]\n{}\n###### error={:#?} #####\n========================================",
            p.to_str(),
            p.error
        );
    }
    println!(
            "################### TOP_KID #####################\n{}\n{:#?}========================================",
            top_kid.to_str(), top_kid.error
        );
    println!(
            "##################################################\n{}\n========================================",
            if let Some(x) = top_kid.root.get_random_child(0.1, 0, &mut params0) { x.to_str(0) } else { "None".to_owned()}
        );

    loop {
        let second_sub = &popln.p[1];
        if let Some(x) = top_kid.root.get_random_child(0.1, 0, &mut params0) {
            println!(
                "parent_fragment===============\n{}\n========================================",
                x.to_str(0)
            );
            if let Some(res_child) = second_sub.root.set_random_child(x, 0.1, 1, &mut params0) {
                println!(
                    "parent===============\n{}\n========================================",
                    second_sub.root.to_str(0)
                );

                println!(
                    "@@@@@@@@@@@@@@@@@@@@@@\n{}\n========================================",
                    res_child.to_str(0)
                );
                break;
            }
        }
    }
}
