mod algorithm;
mod node;

use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    // get the predefined function table
    let table = FloatFnTable::new().table;

    // parameters for building the trees
    let params = nb::BuilderParams::new()
        .max_depth(6)
        .float_range(-1.0, 1.0);

    // randomizer for random number generation
    let mut rng = rand::thread_rng();

    // declare the population model
    // the expression tree to be generated is to take single float as arg and return arg
    let mut popln = ap::Population::new(vec![nb::TypeV::Float], nb::TypeV::Float);

    // generate training data
    let (train_x, train_y) = {
        let mut train_x = Vec::<Vec<nb::Type>>::new();
        let mut train_y = Vec::<nb::Type>::new();
        for _ in 0..256 {
            let x = rng.gen_range(-10.0..=10.0);
            train_x.push(vec![nb::Type::float(x)]);
            train_y.push(nb::Type::float(if (-2.0*PI..=2.0*PI).contains(&x) {
                (x*x.sin()+rng.gen_range(-0.1..=0.1)).exp() //noisy input data
            } else {
                0.0
            }));
        }
        (train_x, train_y)
    };

    // to train the model, set the build table
    popln.set_build_table(table);

    // set expression tree params
    popln.set_params(params);

    // now run the algorithm
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
            .n_iter(8000)
            .mass_extinction_th(50)
            .delta_th(0.08)
            .breed_probability(1.0)
            .mut_probability(1.0)
            .max_allowed_err(1e-6)
            .max_population(2000)
            .compile(),
    );

    // print the top 10 expressions
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
    //print the top kid's expression
    println!(
            "################### TOP_KID #####################\n{}\n{:#?}\n DEPTH={}========================================",
            top_kid.root.get_equation_str(), top_kid.error, top_kid.root.get_max_depth()
        );
}
