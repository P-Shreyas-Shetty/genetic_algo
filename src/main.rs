mod algorithm;
mod node;

use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;
use rand::Rng;

fn main() {
    let table = FloatFnTable::new().table;
    let params = nb::BuilderParams::new().max_depth(5);
    let mut params0 = nb::BuilderParams::new().max_depth(6);
    let mut popln = ap::Population::new(vec![nb::TypeV::Float, nb::TypeV::Float], nb::TypeV::Float);
    let (train_x, train_y) = {
        let mut train_x = Vec::<Vec<nb::Type>>::new();
        let mut train_y = Vec::<nb::Type>::new();
        for _ in 0..100 {
            let (x, y) = (
                params0.randomizer.gen_range(1.0f32..=10.0f32),
                params0.randomizer.gen_range(1.0f32..=10.0f32),
            );
            train_x.push(vec![nb::Type::float(x), nb::Type::float(y)]);
            train_y.push(nb::Type::float(x.powf(y)+y.powf(x)-x.powf(x)-y.powf(y)));
            //train_y.push(nb::Type::float((x*x+y*y)/(x*y)));
        }
        (train_x, train_y)
    };

    popln.set_build_table(table);
    popln.set_params(params);
    let mut top_kid = popln.train(
        &ap::TrainingArgs::new()
            .train_x(&train_x)
            .train_y(&train_y)
            .log_en(true)
            .new_sub_increase_ratio(3, 5)
            .new_sub_intro_period(50)
            .purge_period(5)
            .n_iter(2000)
            .mass_extinction_th(50)
            .delta_th(0.1)
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
            top_kid.to_str(), top_kid.error, top_kid.root.get_max_depth()
        );
}
