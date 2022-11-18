use super::super::node::base as nb;
use super::expr_tree as et;
use rand::Rng;

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
    pub fn generate_mutants(&mut self, num_tries: usize, mut_prob: f32) {
        let initial_population = self.p.len();
        for i in 0..num_tries {
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

    pub fn cross_breed(&mut self, num_tries: usize, breeding_prob: f32) {
        for _ in 0..num_tries {
            let maybe_father_gene = self.p[self.params.randomizer.gen_range(0..num_tries)]
                .root
                .get_random_child(breeding_prob, 0, &mut self.params);
            if let Some(father_gene) = maybe_father_gene {
                let maybe_child = self.p[self.params.randomizer.gen_range(0..num_tries)]
                    .root
                    .set_random_child(father_gene, breeding_prob, 0, &mut self.params);
                if let Some(child) = maybe_child {
                    self.p.push(et::Expr::new(child));
                }
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

    ///Only keep the top expressions with least errors
    /// Keep final_n number of children only
    #[allow(dead_code)]
    pub fn purge_unfit(&mut self, final_n: usize) {
        let l = self.p.len();
        for _ in 0..(l - final_n) {
            self.p.pop();
        }
    }

    ///This is the actual train method
    /// returns the expression tree with least error
    pub fn train(
        &mut self,
        train_x: &[Vec<nb::Type>],
        train_y: &[nb::Type],
        n_iter: usize,
    ) -> et::Expr {
        let num_subs: usize = 128;
        self.init_population(num_subs); //Start with few kids in the beginning
        for i in 0..n_iter {
            self.calc_err(train_x, train_y); //calculate the errors expression tree
            self.sort_population(); //sort the population by error
            self.purge_unfit(num_subs);
            let l = self.p.len() / 2;
            if i != n_iter - 1 {
                self.cross_breed(l, 0.1);
                self.generate_mutants(l, 0.1);
            }
            if i % (n_iter / 5) == 0 {
                self.init_population(num_subs / 2)
            }
            println!("iter {i}");
        }
        self.p[0].clone()
    }
}
