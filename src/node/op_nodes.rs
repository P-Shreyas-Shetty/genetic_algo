use super::base::*;
use super::binary_node_base as bb;

/*
crate::impl_binary_operation_node!(
    name => Add,
    str_ident => "+",
    expression => (lhs, rhs)-> {
        match (rhs, lhs) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(ri + li),
            (Type::Float(ri), Type::Float(li)) => Type::Float(ri + li),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(ri + li),
            _ => panic!("Invalid: Can't add {:?} with {:?}", lhs, rhs),
        }
});
*/

pub struct AddEval;

impl bb::BinOpKind for AddEval {
   const NAME: &'static str = "+";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(ri + li),
            (Type::Float(ri), Type::Float(li)) => Type::Float(ri + li),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(ri + li),
            _ => unreachable!(),
        }
    }
}

/// Add operation: note, both args should be of the same type
pub type Add = bb::BinOpBase<AddEval>;

pub struct SubEval;

impl bb::BinOpKind for SubEval {
    
   const NAME: &'static str = "-";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(li - ri),
            (Type::Float(ri), Type::Float(li)) => Type::Float(li - ri),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(li - ri),
            _ => unreachable!(),
        }
    }
}

/// Sub operation: note, both args should be of the same type
pub type Sub = bb::BinOpBase<SubEval>;

pub struct MulEval;

impl bb::BinOpKind for MulEval {
   const NAME: &'static str = "*";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(li * ri),
            (Type::Float(ri), Type::Float(li)) => Type::Float(li * ri),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(li * ri),
            _ => unreachable!(),
        }
    }
}

/// Mul operation: note, both args should be of the same type
pub type Mul = bb::BinOpBase<MulEval>;

pub struct DivEval;

impl bb::BinOpKind for DivEval {
    
   const NAME: &'static str = "/";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(li / ri),
            (Type::Float(ri), Type::Float(li)) => Type::Float(li / ri),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(li / ri),
            _ => unreachable!(),
        }
    }
}

/// Div operation: note, both args should be of the same type
pub type Div = bb::BinOpBase<DivEval>;

pub struct PowEval;

impl bb::BinOpKind for PowEval {
   const NAME: &'static str = "**";

    fn eval(lhs: Type, rhs: Type) -> Type {
        match (rhs, lhs) {
            (Type::Float(ri), Type::Float(li)) => Type::Float(li.powf(ri)),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(li.pow(ri)),
            _ => unreachable!(),
        }
    }
}

/// Mul operation: note, both args should be of the same type & works only for float and unsigned int
pub type Pow = bb::BinOpBase<PowEval>;
