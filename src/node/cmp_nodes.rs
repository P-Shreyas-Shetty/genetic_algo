///This module defines all the comparison functions
/// all nodes assumes arguments are of same kind
use super::base::*;
use super::binary_node_base as bb;

pub struct EqEval;

impl bb::BinOpKind for EqEval {
    const NAME: &'static str = "==";
    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(li == ri),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri == li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri == li),
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri == li),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
pub type Eq = bb::BinOpBase<EqEval>;

pub struct NEqEval;

impl bb::BinOpKind for NEqEval {
    const NAME: &'static str = "!=";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(li != ri),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri != li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri != li),
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri != li),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
pub type NEq = bb::BinOpBase<NEqEval>;

pub struct GtEval;

impl bb::BinOpKind for GtEval {
    const NAME: &'static str = ">";
    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(li & !ri),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri > li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri > li),
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri > li),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
pub type Gt = bb::BinOpBase<GtEval>;

pub struct GteEval;

impl bb::BinOpKind for GteEval {
    const NAME: &'static str = ">=";
    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(li >= ri),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri >= li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri >= li),
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri >= li),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
pub type Gte = bb::BinOpBase<GteEval>;

pub struct LtEval;

impl bb::BinOpKind for LtEval {
    const NAME: &'static str = "<";
    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(!li & ri),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri < li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri < li),
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri < li),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
pub type Lt = bb::BinOpBase<LtEval>;

pub struct LteEval;

impl bb::BinOpKind for LteEval {
    const NAME: &'static str = "<=";
    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(li <= ri),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri <= li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri <= li),
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri <= li),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
pub type Lte = bb::BinOpBase<LteEval>;
