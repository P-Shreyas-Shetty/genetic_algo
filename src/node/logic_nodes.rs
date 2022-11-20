#![allow(dead_code)]
use super::base::*;
use super::binary_node_base as bb;
use super::unary_node_base as ub;


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


pub struct NotEval {}

impl ub::UnaryOpKind for NotEval {
   const ARG_TYPE: TypeV = TypeV::Bool;
   const RTYPE: TypeV = TypeV::Bool;
   const NAME: &'static str = "~"; 

   fn eval(input: Type) -> Type {
       if let Type::Bool(b) = input {
        Type::Bool(!b)
       }
       else {
        unreachable!()
       }
   }
}

/// Boolean not operation
pub type Not = ub::UnaryOpBase<NotEval>;

