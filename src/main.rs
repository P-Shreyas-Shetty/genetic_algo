mod algorithm;
mod node;

use algorithm::population as ap;
use node::base as nb;
use node::btables::FloatFnTable;

fn main() {
    let table = FloatFnTable::new().table;
    let mut params = nb::BuilderParams::new().max_depth(6);
    let mut popln = ap::Population::new(vec![nb::TypeV::Float, nb::TypeV::Float], nb::TypeV::Float);
    popln.set_build_table(table);
    popln.set_params(params);
    popln.init_population(4);

    for p in popln.p {
        let typecheck = if let Ok(_) = p.type_check() {
            true
        } else {
            false
        };
        println!(
            "{}\n######typecheck={}#####\n========================================",
            p.to_str(),
            typecheck
        );
    }
}
