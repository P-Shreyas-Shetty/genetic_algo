use super::super::node::base as nb;
use super::expr_tree as et;

#[allow(dead_code)]
pub struct Population {
    //All the nodes in the population
    pub p: Vec<et::Expr>,
    //Parameters used for building and mutating nodes
    pub params: nb::BuilderParams,
    pub builder_table: nb::BuilderTable,
    pub arg_types: Vec<nb::TypeV>,
    pub ret_type: nb::TypeV,
}

impl Population {
    #[allow(dead_code)]
    pub fn new(arg_types: Vec<nb::TypeV>, ret_type: nb::TypeV) -> Population {
        Population {
            p: vec![],
            params: nb::BuilderParams::new(), //default params
            arg_types,
            ret_type,
            builder_table: nb::BuilderTable::new(), //dummy, empty build table
        }
    }

    #[allow(dead_code)]
    pub fn set_build_table(&mut self, build_table: nb::BuilderTable) {
        self.builder_table = build_table;
    }

    #[allow(dead_code)]
    pub fn set_params(&mut self, params: nb::BuilderParams) {
        self.params = params;
    }

    #[allow(dead_code)]
    pub fn init_population(&mut self, num_subs: usize) {
        for _ in 0..num_subs {
            self.p.push(et::Expr::random(
                self.arg_types.clone(),
                self.ret_type,
                &self.builder_table,
                &mut self.params,
            ))
        }
    }

    ///Sorts the population accordig to fitness,
    /// if the fitness is uncalculated, panics
    #[allow(dead_code)]
    pub fn sort_population(&mut self) {
        todo!()
    }
}
