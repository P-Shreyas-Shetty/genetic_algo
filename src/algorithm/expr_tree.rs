#![allow(dead_code)]
use super::super::node::base as nb;

pub enum Error {
    Uncalculated,
    Val(f32),
}

pub struct Expr {
    pub root: nb::NodeRef,
    arg_types: Vec<nb::TypeV>,
    rtype: nb::TypeV,
}

impl Expr {
    pub fn new(root: nb::NodeRef) -> Expr {
        Expr {
            arg_types: root.get_arg_types().to_vec(),
            rtype: root.get_rtype(),
            root,
        }
    }
    pub fn random<'a>(
        arg_types: Vec<nb::TypeV>,
        rtype: nb::TypeV,
        builder_table: &nb::BuilderTable,
        params: &'a mut nb::BuilderParams,
    ) -> Expr {
        Expr {
            root: builder_table
                .get_rand_node(0, rtype, params)
                .build_random_node(builder_table, &arg_types, rtype, 0, params),
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
}
