mod algorithm;
mod node;

use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    let table = FloatFnTable::new().table;
    let params = nb::BuilderParams::new().max_depth(5);
    let mut params0 = nb::BuilderParams::new().max_depth(6);
    let mut popln = ap::Population::new(vec![nb::TypeV::Float, nb::TypeV::Float], nb::TypeV::Float);
    let (train_x, train_y) = {
        let mut train_x = Vec::<Vec<nb::Type>>::new();
        let mut train_y = Vec::<nb::Type>::new();
        for _ in 0..256 {
            let (x, y) = (
                params0.randomizer.gen_range(0.0f32..=PI),
                params0.randomizer.gen_range(0.0f32..=PI),
            );
            train_x.push(vec![nb::Type::float(x), nb::Type::float(y)]);
            //train_y.push(nb::Type::float(x.powf(y)+y.powf(x)-x.powf(x)-y.powf(y)));
            train_y.push(nb::Type::float(
                (f32::exp(f32::sin(x)) + f32::exp(f32::cos(x))).powf(x * y)/PI,
            ));
        }
        (train_x, train_y)
    };

    popln.set_build_table(table);
    popln.set_params(params);
    let mut top_kid = popln.train(
        &ap::TrainingArgs::new()
            .train_x(&train_x)
            .train_y(&train_y)
            .n_subs(200)
            .log_en(true)
            .exec_time_log_en(true)
            .new_sub_increase_ratio(1, 5)
            .new_sub_intro_period(20)
            .purge_period(5)
            .n_iter(8000)
            .mass_extinction_th(50)
            .delta_th(0.08)
            .breed_probability(1.0)
            .max_allowed_err(1e-6)
            .compile(),
    );
    top_kid.calc_err(&train_x, &train_y);

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
