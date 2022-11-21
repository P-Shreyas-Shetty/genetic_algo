use super::super::node::base as nb;
use super::expr_tree::{self as et, Error};
use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::Rng;

pub struct TrainingArgs<'a> {
    /// training data input
    pub train_x: Option<&'a [Vec<nb::Type>]>,
    ///train data output
    pub train_y: Option<&'a [nb::Type]>,
    /// number of subjects in population at the end of iteration
    pub n_subs: usize,
    /// after `purge_period` iterations, unfit children are purged from population
    pub purge_period: usize,
    /// total number iterations
    pub n_iter: usize,
    /// probability of mutation. This increases logistically as you go down the tree
    pub mut_probability: f32,
    /// probability certain branch chosen for cross-breeding. This increases logistically as you go down the tree
    pub breed_probability: f32,
    /// every `new_sub_intro_period` iterations, completely new random subjects are introduced into the population
    pub new_sub_intro_period: usize,
    /// n_subs*new_sub_increase_ratio.0/*new_sub_increase_ratio.1 number of
    /// new subjects are added to population every `new_sub_intro_period` iterations
    pub new_sub_increase_ratio: (usize, usize),
    /// ratio of top children chosen for breeding
    pub top_children_ratio: (usize, usize),
    /// if the minimum error doesn't change for `mass_extinction_th` iterations,
    /// then trigger a mass extinction. This will retain only the top child
    /// in the population & introduce `n_subs` number of new individuals
    pub mass_extinction_th: usize,
    /// minim difference in error from one iteration to next iteraton
    /// expected it to be considered "changed"
    pub delta_th: f32,
    /// enables logging
    pub log_en: bool,
}

impl<'a> TrainingArgs<'a> {
    pub fn new() -> Self {
        Self {
            train_x: None,
            train_y: None,
            n_subs: 128,
            n_iter: 1000,
            log_en: false,
            new_sub_intro_period: 5,
            new_sub_increase_ratio: (1, 2),
            top_children_ratio: (1, 2),
            mut_probability: 0.1,
            breed_probability: 0.1,
            purge_period: 1,
            delta_th: 0.01,
            mass_extinction_th: 50,
        }
    }
    #[allow(dead_code)]
    /// training data input
    pub fn train_x(mut self, val: &'a [Vec<nb::Type>]) -> Self {
        self.train_x = Some(val);
        self
    }
    #[allow(dead_code)]
    ///train data output
    pub fn train_y(mut self, val: &'a [nb::Type]) -> Self {
        self.train_y = Some(val);
        self
    }
    #[allow(dead_code)]
    /// number of subjects in population at the end of iteration
    pub fn n_subs(mut self, val: usize) -> Self {
        self.n_subs = val;
        self
    }
    #[allow(dead_code)]
    /// total number iterations
    pub fn n_iter(mut self, val: usize) -> Self {
        self.n_iter = val;
        self
    }
    #[allow(dead_code)]
    /// probability of mutation. This increases logistically as you go down the tree
    pub fn mut_probability(mut self, val: f32) -> Self {
        self.mut_probability = val;
        self
    }
    #[allow(dead_code)]
    /// probability certain branch chosen for cross-breeding. This increases logistically as you go down the tree
    pub fn breed_probability(mut self, val: f32) -> Self {
        self.breed_probability = val;
        self
    }
    #[allow(dead_code)]
    /// enables logging
    pub fn log_en(mut self, val: bool) -> Self {
        self.log_en = val;
        self
    }

    #[allow(dead_code)]
    /// every `new_sub_intro_period` iterations, completely new random subjects are introduced into the population
    pub fn new_sub_intro_period(mut self, val: usize) -> Self {
        self.new_sub_intro_period = val;
        self
    }

    #[allow(dead_code)]
    /// n_subs*new_sub_increase_ratio.0/*new_sub_increase_ratio.1 number of
    /// new subjects are added to population every `new_sub_intro_period` iterations
    pub fn new_sub_increase_ratio(mut self, num: usize, den: usize) -> Self {
        self.new_sub_increase_ratio = (num, den);
        self
    }
    #[allow(dead_code)]
    /// ratio of top children chosen for breeding
    pub fn top_children_ratio(mut self, num: usize, den: usize) -> Self {
        self.top_children_ratio = (num, den);
        self
    }
    #[allow(dead_code)]
    /// after `purge_period` iterations, unfit children are purged from population
    /// will increase memory consumption and training time if its too high
    pub fn purge_period(mut self, val: usize) -> Self {
        self.purge_period = val;
        self
    }
    #[allow(dead_code)]
    /// if the minimum error doesn't change for `mass_extinction_th` iterations,
    /// then trigger a mass extinction. This will retain only the top child
    /// in the population & introduce `n_subs` number of new individuals
    pub fn mass_extinction_th(mut self, val: usize) -> Self {
        self.mass_extinction_th = val;
        self
    }
    #[allow(dead_code)]
    /// minim difference in error from one iteration to next iteraton
    /// expected it to be considered "changed"
    pub fn delta_th(mut self, val: f32) -> Self {
        self.delta_th = val;
        self
    }
    /// checks the argument for correctness
    pub fn compile(self) -> Self {
        if self.train_x.is_none() {
            panic!("Required: train_x");
        }
        if self.train_y.is_none() {
            panic!("Required: train_y");
        }
        self
    }
}

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
    pub fn generate_mutants(&mut self, num_tries: usize, mut_prob: f32, log_en: bool) {
        let initial_population = self.p.len();
        let mut n_success = 0usize;
        let weights: Vec<_> = (0..initial_population).rev().collect();
        let weighted_dist = WeightedIndex::new(&weights).unwrap();
        for _ in 0..num_tries {
            //randomly do a weighted selection
            let p = &self.p[weighted_dist.sample(&mut self.params.randomizer)];
            let mut_prob = if let Error::Err { real: r, nan: _n } = p.error {
                r
            } else {
                mut_prob
            };
            let maybe_mutant = p.root.mutant_copy(
                mut_prob,
                0,
                &self.arg_types,
                &self.builder_table,
                &mut self.params,
            );
            if let Some(s) = maybe_mutant {
                self.p.push(et::Expr::new(s));
                n_success += 1;
            }
        }
        if log_en {
            println!(
                "    generate_mutants ::  num_tries={num_tries}, new_children_added={n_success}"
            );
        }
    }

    pub fn cross_breed(&mut self, num_tries: usize, breeding_prob: f32, log_en: bool) {
        let mut n_success = 0usize;
        let initial_population = self.p.len();
        let weights: Vec<_> = (0..initial_population).rev().collect();
        let weighted_dist = WeightedIndex::new(&weights).unwrap();
        for _ in 0..num_tries {
            let maybe_father_gene = self.p[weighted_dist.sample(&mut self.params.randomizer)]
                .root
                .get_random_child(breeding_prob, 0, &mut self.params);
            if let Some(father_gene) = maybe_father_gene {
                let maybe_child = self.p[self.params.randomizer.gen_range(0..initial_population)]
                    .root
                    .set_random_child(father_gene, breeding_prob, 0, &mut self.params);
                if let Some(child) = maybe_child {
                    self.p.push(et::Expr::new(child));
                    n_success += 1;
                }
            }
        }
        if log_en {
            println!("    cross_breed ::  num_tries={num_tries}, new_children_added={n_success}");
        }
    }

    pub fn prune_population(&mut self) {
        for p in self.p.iter_mut() {
            p.prune()
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
    pub fn sort_population(&mut self, log_en: bool) {
        self.p.sort_by(|a, b| a.error.cmp(&b.error));
        if log_en {
            if let Error::Err { real, nan } = self.p[0].error {
                println!("    sort_population: minimum_error in population := real_err: {real}, nan: {nan}");
            }
        }
    }

    ///Only keep the top expressions with least errors
    /// Keep final_n number of children only
    #[allow(dead_code)]
    pub fn purge_unfit(&mut self, final_n: usize, log_en: bool) {
        let l = self.p.len();
        for _ in 0..(l - final_n) {
            self.p.pop();
        }
        if log_en {
            println!("    purge_unfit :: final population = {}", self.p.len());
        }
    }

    ///This is the actual train method
    /// returns the expression tree with least error
    pub fn train(&mut self, args: &TrainingArgs) -> et::Expr {
        let num_subs = args.n_subs;
        let n_iter = args.n_iter;
        let train_x = args.train_x.unwrap();
        let train_y = args.train_y.unwrap();
        let breed_prob = args.breed_probability;
        let mut_prob = args.mut_probability;
        let mut minim_error = Error::Uncalculated;
        let mut stagnant_cycles = 0usize;
        self.init_population(num_subs); //Start with few kids in the beginning
        for i in 0..n_iter {
            if args.log_en {
                println!("Log: n_iter {i}")
            }
            self.prune_population();
            self.calc_err(train_x, train_y); //calculate the errors expression tree
            self.sort_population(args.log_en); //sort the population by error
            if i % args.purge_period == 0 {
                self.purge_unfit(num_subs, args.log_en);
            }
            let l = (self.p.len() * args.top_children_ratio.0) / args.top_children_ratio.1;
            if i % args.new_sub_intro_period == 0 {
                self.init_population(
                    (num_subs * args.new_sub_increase_ratio.0) / args.new_sub_increase_ratio.1,
                );
            }
            if i != n_iter - 1 {
                self.cross_breed(l, breed_prob, args.log_en);
                self.generate_mutants(l, mut_prob, args.log_en);
            }

            if args.log_en {
                println!();
            }
            if let Error::Err { real, nan } = self.p[0].error {
                if real == 0.0 && nan == 0.0 {
                    break;
                }
                if let Error::Err {
                    real: min_real,
                    nan: min_nan,
                } = minim_error
                {
                    let denom_real_err = if real == 0.0 { 1.0 } else { real };
                    let denom_nan_err = if nan == 0.0 { 1.0 } else { nan };
                    if (min_real - real).abs() / denom_real_err <= args.delta_th
                        && (min_nan - nan).abs() / denom_nan_err <= args.delta_th
                    {
                        stagnant_cycles += 1;
                    } else {
                        minim_error = Error::Err { real, nan };
                        stagnant_cycles = 0;
                    }
                } else {
                    minim_error = self.p[0].error;
                }
            }
            //if minimum error remains unchanged for long time,
            //trigger a mass extinction. Purge all but the top child
            //and fill the population with new random children
            if stagnant_cycles >= args.mass_extinction_th {
                if args.log_en {
                    println!("   ### Triggering mass extinction ##");
                }
                self.purge_unfit(1, args.log_en);
                self.init_population(num_subs - 1);
                stagnant_cycles = 0;
            }
        }
        self.p[0].clone()
    }
}
