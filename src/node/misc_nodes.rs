use super::base::*;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Cond {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub cond: NodeRef,
    pub iftrue: NodeRef,
    pub iffalse: NodeRef,
}

impl Cond {
    pub fn make(cond: NodeRef, iftrue: NodeRef, iffalse: NodeRef) -> NodeRef {
        let rtype = iftrue.get_rtype();
        assert_eq!(iftrue.get_rtype(), iffalse.get_rtype());
        assert_eq!(cond.get_rtype(), TypeV::Bool);
        Box::new(Cond {
            rtype,
            arg_types: vec![cond.get_rtype(), iftrue.get_rtype(), iffalse.get_rtype()],
            cond,
            iftrue,
            iffalse,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Cond {
            rtype,
            arg_types,
            cond: Null::zero(rtype),
            iftrue: Null::zero(rtype),
            iffalse: Null::zero(rtype),
        })
    }
}
impl Node for Cond {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "?\n"
            + &self.cond.to_str(indent + 1)
            + "\n"
            + &self.iftrue.to_str(indent + 1)
            + &self.iffalse.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
        let c = self.cond.eval(args);
        let t = self.iftrue.eval(args);
        let f = self.iffalse.eval(args);
        match (c, t, f) {
            (Type::Bool(ci), Type::Float(ti), Type::Float(fi)) => if ci { Type::Float(ti) } else {Type::Float(fi)},
            (Type::Bool(ci), Type::Int(ti), Type::Int(fi)) => if ci { Type::Int(ti) } else {Type::Int(fi)},
            (Type::Bool(ci), Type::UInt(ti), Type::UInt(fi)) => if ci { Type::UInt(ti) } else {Type::UInt(fi)},
            _ => panic!("Invalid: Cond must be bool (Got {:?}) Branches must be of same type (Got {:?} and {:?})", c, t, f)
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef) {
        match child_index {
            0 => self.cond = child,
            1 => self.iftrue = child,
            2 => self.iffalse = child,
            _ => unreachable!(),
        }
    }

    fn get_child(&self, child_index: usize) -> &NodeRef {
        match child_index {
            0 => &self.cond,
            1 => &self.iftrue,
            2 => &self.iffalse,
            _ => unreachable!(),
        }
    }
    fn get_type_zero(&self) -> NodeRef {
        Self::zero(self.rtype, self.arg_types.clone())
    }

    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams,
    ) -> NodeRef {
        let mut node = Self::get_type_zero(self);
        let cond = build_table
            .get_rand_node(depth + 1, TypeV::Bool, params)
            .build_random_node(build_table, arg_types, TypeV::Bool, depth + 1, params);
        let iftrue = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        let iffalse = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        node.set_child(0, cond);
        node.set_child(1, iftrue);
        node.set_child(1, iffalse);
        node
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        if (self.cond.get_rtype() == TypeV::Bool)
            && (self.iftrue.get_rtype() == self.rtype)
            && (self.iffalse.get_rtype() == self.rtype)
        {
            self.cond.type_check()?;
            self.iftrue.type_check()?;
            self.iffalse.type_check()?;
            Ok(())
        } else {
            Err(TypeErr {msg: format!("Cond required argument of type ({:#?}, {:#?}, {:#?}); Got ({:#?}, {:#?}, {:#?})!!", TypeV::Bool, self.rtype, self.rtype, self.cond.get_rtype(), self.iftrue.get_rtype(), self.iffalse.get_rtype())})
        }
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable,
        params: &'a mut BuilderParams,
    ) -> Option<NodeRef> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, node_depth) {
            Some(self.build_random_node(
                build_table,
                arg_types,
                self.get_rtype(),
                node_depth,
                params,
            ))
        } else {
            let cond =
                self.cond
                    .mutant_copy(probability, node_depth + 1, arg_types, build_table, params);
            let iftrue = self.iftrue.mutant_copy(
                probability,
                node_depth + 1,
                arg_types,
                build_table,
                params,
            );
            let iffalse = self.iffalse.mutant_copy(
                probability,
                node_depth + 1,
                arg_types,
                build_table,
                params,
            );
            match (cond, iftrue, iffalse) {
                (None, None, None) => None,
                (cond, iftrue, iffalse) => {
                    let cond_vld = if let Some(c) = cond {
                        c
                    } else {
                        self.cond.deep_copy()
                    };
                    let iftrue_vld = if let Some(t) = iftrue {
                        t
                    } else {
                        self.iftrue.deep_copy()
                    };
                    let iffalse_vld = if let Some(f) = iffalse {
                        f
                    } else {
                        self.iffalse.deep_copy()
                    };
                    let mut ret = Self::zero(self.rtype, self.arg_types.clone());
                    ret.set_child(0, cond_vld);
                    ret.set_child(1, iftrue_vld);
                    ret.set_child(2, iffalse_vld);
                    Some(ret)
                }
            }
        }
    }
    fn deep_copy(&self) -> NodeRef {
        Self::make(
            self.cond.deep_copy(),
            self.iftrue.deep_copy(),
            self.iffalse.deep_copy(),
        )
    }
    fn get_random_child(
        &self,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth) {
            Some(self.deep_copy())
        } else {
            let maybe_cond = self.cond.get_random_child(probability, depth + 1, params);
            let maybe_iftrue = self.iftrue.get_random_child(probability, depth + 1, params);
            let maybe_iffalse = self
                .iffalse
                .get_random_child(probability, depth + 1, params);

            match (maybe_cond, maybe_iftrue, maybe_iffalse) {
                (None, None, None) => None,
                (maybe_cond, maybe_iftrue, maybe_iffalse) => {
                    let valid_child: Vec<i32> = vec![
                        maybe_cond.is_some().into(),
                        maybe_iftrue.is_some().into(),
                        maybe_iffalse.is_some().into(),
                    ];
                    let idx = valid_child
                        .choose_weighted(&mut params.randomizer, |x| *x)
                        .expect("Random selection failed");
                    if *idx == 0 {
                        maybe_cond
                    } else if *idx == 1 {
                        maybe_iftrue
                    } else if *idx == 2 {
                        maybe_iffalse
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }

    fn set_random_child(
        &self,
        new_node: NodeRef,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        if new_node.get_rtype() == self.get_rtype()
            && params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth)
        {
            if new_node.get_rtype() == self.get_rtype() {
                Some(new_node)
            } else {
                None
            }
        } else {
            unimplemented!()
        }
    }

    fn get_name(&self) -> &'static str {
        "Cond"
    }

    fn prune(&self) -> NodeRef {
        Self::make(self.cond.prune(), self.iftrue.prune(), self.iffalse.prune())
    }

    fn get_max_depth(&self) -> usize {
        let cond_depth = self.cond.get_max_depth();
        let iftrue_depth = self.iftrue.get_max_depth();
        let iffalse_depth = self.iffalse.get_max_depth();
        let branch_max_depth = usize::max(iffalse_depth, iftrue_depth);
        usize::max(cond_depth, branch_max_depth) + 1
    }
}
