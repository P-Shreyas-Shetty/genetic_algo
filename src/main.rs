mod algorithm;
mod node;

use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;
use rand::Rng;
//use std::f32::consts::PI;

fn main() {
    let table = FloatFnTable::new().table;
    let mut params = nb::BuilderParams::new().max_depth(9).float_range(-1.0, 1.0);
    params.set_seed(123);
    let mut params0 = nb::BuilderParams::new().max_depth(6);
    params0.set_seed(345);
    //let mut popln = ap::Population::new(vec![nb::TypeV::Float, nb::TypeV::Float], nb::TypeV::Float);
    let mut popln = ap::Population::new(vec![nb::TypeV::Float], nb::TypeV::Float);
    let (train_x, train_y) = {
        let mut train_x = Vec::<Vec<nb::Type>>::new();
        let mut train_y = Vec::<nb::Type>::new();
        for _ in 0..256 {
            //let (x, y) = (
            //    params0.randomizer.gen_range(-5.0..=5.0),
            //    params0.randomizer.gen_range(-5.0..=5.0),
            //);
            let x = params0.randomizer.gen_range(-10.0..=10.0);
            //train_x.push(vec![nb::Type::float(x), nb::Type::float(y)]);
            train_x.push(vec![nb::Type::float(x)]);
            //train_y.push(nb::Type::float(x.powf(y)+y.powf(x)-x.powf(x)-y.powf(y)));
            train_y.push(nb::Type::float(if (-1.0..=1.0).contains(&x) {
                x.abs() * 6.0
            } else if (2.0..=3.0).contains(&x) || (-3.0..=-2.0).contains(&x) {
                x * x
            } else {
                0.0
            }));
        }
        (train_x, train_y)
    };

    popln.set_build_table(table);
    popln.set_params(params);
    let top_kid = popln.train(
        &ap::TrainingArgs::new()
            .train_x(&train_x)
            .train_y(&train_y)
            .n_subs(256)
            .log_en(true)
            .exec_time_log_en(true)
            .new_sub_increase_ratio(1, 5)
            .new_sub_intro_period(20)
            .purge_period(6)
            .n_iter(5000)
            .mass_extinction_th(50)
            .delta_th(0.08)
            .breed_probability(1.0)
            .mut_probability(1.0)
            .max_allowed_err(1e-6)
            .max_population(2000)
            .compile(),
    );

    
    for (i, p) in popln.p.iter().enumerate() {
        println!(
            ">>>[{i}]\n{}\n###### error={:#?} #####\n========================================",
            p.to_str(),
            p.error
        );
        if i > 10 {
            break;
        }
    }
    println!(
            "################### TOP_KID #####################\n{}\n{:#?}\n DEPTH={}========================================",
            top_kid.root.get_equation(), top_kid.error, top_kid.root.get_max_depth()
        );
}
