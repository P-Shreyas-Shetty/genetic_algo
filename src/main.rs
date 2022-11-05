//mod algorithm;
mod node;

//use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;

fn main() {/* 
    let table = FloatFnTable::new().table;
    let params = nb::BuilderParams::new().max_depth(6);
    let mut popln = ap::Population::new(vec![nb::TypeV::Float, nb::TypeV::Float], nb::TypeV::Float);
    let train_x = vec![
        vec![nb::Type::float(1.0), nb::Type::float(2.0)],
        vec![nb::Type::float(3.0), nb::Type::float(4.0)],
        vec![nb::Type::float(5.0), nb::Type::float(6.0)],
        vec![nb::Type::float(7.0), nb::Type::float(8.0)],
        vec![nb::Type::float(-3.0), nb::Type::float(4.0)],
    ];
    let train_y = vec![
        nb::Type::float(3.0),
        nb::Type::float(7.0),
        nb::Type::float(11.0),
        nb::Type::float(15.0),
        nb::Type::float(1.0),
    ];
    popln.set_build_table(table);
    popln.set_params(params);
    popln.init_population(4);

    popln.generate_mutants(4, 0.01);

    popln.calc_err(&train_x, &train_y);
    popln.sort_population();
    for (i, p) in popln.p.iter().enumerate() {
        println!(
            ">>>[{i}]\n{}\n###### error={:#?} #####\n========================================",
            p.to_str(),
            p.error
        );
    }*/
}
