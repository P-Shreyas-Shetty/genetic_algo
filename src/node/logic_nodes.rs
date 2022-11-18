#![allow(dead_code)]
use super::base::*;
use super::binary_node_base as bb;
use rand::Rng;


pub struct AndEval;

impl bb::BinOpKind for AndEval {
   const NAME: &'static str = "&";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(li && ri),
            (Type::Int(ri), Type::Int(li)) => Type::Int(ri & li),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(ri & li),
            _ => unreachable!(),
        }
    }
}

pub type And = bb::BinOpBase<AndEval>;

pub struct OrEval;

impl bb::BinOpKind for OrEval {
   const NAME: &'static str = "|";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(li && ri),
            (Type::Int(ri), Type::Int(li)) => Type::Int(ri & li),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(ri & li),
            _ => unreachable!(),
        }
    }
}

pub type Or = bb::BinOpBase<OrEval>;



pub struct Not {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
}

impl Not {
    pub fn make(rhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), TypeV::Bool);
        Box::new(Not {
            rtype,
            arg_types: vec![rhs.get_rtype()],
            rhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Not {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
        })
    }
}

impl Node for Not {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent) + "!\n" + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
        let r = self.rhs.eval(args);
        match r {
            Type::Bool(ri) => Type::Bool(!ri),
            _ => panic!("Invalid: Can't Compl {:?} ", r),
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
            0 => self.rhs = child,
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
        let rhs = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        node.set_child(0, rhs);
        node
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        if self.rhs.get_rtype() == TypeV::Bool {
            self.rhs.type_check()?;
            self.rhs.type_check()?;
            Ok(())
        } else {
            Err(TypeErr {
                msg: format!(
                    "Not required argument of type ({:#?}); Got ({:#?})!!",
                    TypeV::Bool,
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef {
        Self::make(self.rhs.deep_copy())
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
            let mut ret = Self::zero(self.rtype, self.arg_types.clone());
            let rhs = self.rhs.mutant_copy(
                probability,
                node_depth + 1,
                arg_types,
                build_table,
                params,
            )?; //If mutation return None, then return None for parent node's mutation
            ret.set_child(1, rhs);
            Some(ret)
        }
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
            self.rhs.get_random_child(probability, depth + 1, params)
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
            Some(new_node)
        } else {
            let child = self
                .rhs
                .set_random_child(new_node, probability, depth + 1, params)?;
            Some(Self::make(child))
        }
    }
}
