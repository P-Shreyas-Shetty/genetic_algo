#![allow(dead_code)]

use super::super::node::base as nb;
use std::cmp::Ordering;

/// This type represents the Error calculated
/// by default, its Uncalculated
/// on applying test data on the expressions,
/// the Error is calculated
/// It is a vector of actual real value errors
/// and number of nans it outputs
/// TODO: Decide how to weight the nans
#[derive(Debug, Clone, Copy)]
pub enum Error {
    Uncalculated,
    Err { real: f32, nan: f32 },
}

impl Error {
    pub fn cmp(&self, other: &Error) -> Ordering {
        match (&self, other) {
            (_, Error::Uncalculated) => {
                unreachable!()
            }
            (Error::Uncalculated, _) => {
                unreachable!()
            }
            (
                Error::Err {
                    real: real0,
                    nan: n0,
                },
                Error::Err {
                    real: real1,
                    nan: n1,
                },
            ) => {
                if *n0 != *n1 {
                    n0.total_cmp(n1)
                } else {
                    real0.total_cmp(real1)
                }
            }
        }
    }
}

pub struct Expr {
    pub root: nb::NodeRef,
    pub error: Error,
    arg_types: Vec<nb::TypeV>,
    rtype: nb::TypeV,
}

impl Expr {
    pub fn new(root: nb::NodeRef) -> Expr {
        Expr {
            arg_types: root.get_arg_types().to_vec(),
            rtype: root.get_rtype(),
            error: Error::Uncalculated,
            root,
        }
    }

    pub fn clone(&self) -> Self {
        let root = self.root.deep_copy();
        Expr {
            root,
            error: self.error,
            arg_types: self.arg_types.clone(),
            rtype: self.rtype,
        }
    }
    pub fn random(
        arg_types: Vec<nb::TypeV>,
        rtype: nb::TypeV,
        builder_table: &nb::BuilderTable,
        params: &'_ mut nb::BuilderParams,
    ) -> Expr {
        Expr {
            root: builder_table
                .get_rand_node(0, rtype, params)
                .build_random_node(builder_table, &arg_types, rtype, 0, params),
            error: Error::Uncalculated,
            arg_types,
            rtype,
        }
    }

    pub fn to_str(&self) -> String {
        self.root.to_str(0)
    }

    pub fn type_check(&self) -> Result<(), nb::TypeErr> {
        self.root.type_check()
    }

    /// calculates the error of an expression tree.
    /// Takes input and expected outputs as argument,
    /// calculates the output from the given input;
    /// sets the `error` field of the tree with average
    /// error over all the given inputs.
    pub fn calc_err(
        &mut self,
        train_x: &[Vec<nb::Type>],
        train_y: &[nb::Type],
        
    ) {
        let mut err_real: f32 = 0.0;
        let mut err_nan: f32 = 0.0;

        for i in 0..train_x.len() {
           
            let e = match (self.root.eval(&train_x[i]), train_y[i]) {
                (nb::Type::Float(pred_y_dat), nb::Type::Float(train_y_dat)) => {
                    if train_y_dat != 0.0 {
                        ((pred_y_dat - train_y_dat) / train_y_dat).abs()
                    } else {
                        pred_y_dat.abs()
                    }
                }
                (_, _) => unimplemented!(),
            };
            if e.is_finite() {
                err_real += e;
            } else {
                err_nan += 1.0;
            }
        }
        self.error = Error::Err {
            real: err_real / train_y.len() as f32,
            nan: err_nan / train_y.len() as f32,
        }
    }

    pub fn prune(&mut self) {
        self.root = self.root.prune();
    }
}
