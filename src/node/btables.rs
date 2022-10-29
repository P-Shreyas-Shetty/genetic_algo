use super::base::*;
use super::math_nodes as math;
use super::op_nodes as ops;

/// function table for floating point numbers
pub struct FloatFnTable {
    pub table: BuilderTable,
}

impl FloatFnTable {
    pub fn new() -> Self {
        let mut b = Self {
            table: BuilderTable::new(),
        };
        b.table.push(TypeV::Float, math::Sin::zero());
        b.table.push(TypeV::Float, math::Cos::zero());
        b.table.push(TypeV::Float, math::Tan::zero());
        b.table.push(TypeV::Float, math::Exp::zero());
        b.table.push(TypeV::Float, math::Log::zero());
        b.table.push(TypeV::Float, math::Abs::zero());
        b.table.push(
            TypeV::Float,
            ops::Add::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        b.table.push(
            TypeV::Float,
            ops::Sub::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        b.table.push(
            TypeV::Float,
            ops::Mul::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        b.table.push(
            TypeV::Float,
            ops::Div::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        b.table.push(
            TypeV::Float,
            ops::Pow::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );

        return b;
    }
}
