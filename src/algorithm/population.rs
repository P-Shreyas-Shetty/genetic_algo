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

    #[allow(dead_code)]
    pub fn generate_mutants(&mut self, num_subs: usize, mut_prob: f32) {
        let initial_population = self.p.len();
        for i in 0..num_subs {
            let p = &self.p[i % initial_population];
            let maybe_mutant = p.root.mutant_copy(
                mut_prob,
                0,
                &self.arg_types,
                &self.builder_table,
                &mut self.params,
            );
            if let Some(s) = maybe_mutant {
                self.p.push(et::Expr::new(s));
            }
        }
    }

    #[allow(dead_code)]
    pub fn calc_err(&mut self, train_x: &[Vec<nb::Type>], train_y: &[nb::Type]) {
        for p in self.p.iter_mut() {
            p.calc_err(train_x, train_y);
        }
    }
    ///Sorts the population accordig to fitness,
    /// if the fitness is uncalculated, panics
    #[allow(dead_code)]
    pub fn sort_population(&mut self) {
        self.p.sort_by(|a, b| a.error.cmp(&b.error));
    }
}
