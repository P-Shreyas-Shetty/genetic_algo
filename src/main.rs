mod algorithm;
mod node;

use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;

fn main() {
    let table = FloatFnTable::new().table;
    let params = nb::BuilderParams::new().max_depth(6);
    let mut popln = ap::Population::new(vec![nb::TypeV::Float, nb::TypeV::Float], nb::TypeV::Float);
    popln.set_build_table(table);
    popln.set_params(params);
    popln.init_population(4);

    popln.generate_mutants(4, 0.01);
    for (i, p) in popln.p.iter().enumerate() {
        let typecheck = if let Ok(_) = p.type_check() {
            true
        } else {
            false
        };
        println!(
            ">>>[{i}]\n{}\n###### typecheck={} #####\n========================================",
            p.to_str(),
            typecheck
        );
    }
}
