use super::base::*;
use super::math_nodes as math;
use super::op_nodes as ops;

/// function table for floating point numbers
pub struct FloatFnTable {
    pub table: BuilderTable,
}

impl FloatFnTable {
    pub fn new() -> Self {
        let mut float_fn_table = Self {
            table: BuilderTable::new(),
        };
        float_fn_table.table.push(TypeV::Float, math::Sin::zero());
        float_fn_table.table.push(TypeV::Float, math::Cos::zero());
        float_fn_table.table.push(TypeV::Float, math::Tan::zero());
        float_fn_table.table.push(TypeV::Float, math::ASin::zero());
        float_fn_table.table.push(TypeV::Float, math::ACos::zero());
        float_fn_table.table.push(TypeV::Float, math::ATan::zero());
        float_fn_table.table.push(TypeV::Float, math::Sinh::zero());
        float_fn_table.table.push(TypeV::Float, math::Cosh::zero());
        float_fn_table.table.push(TypeV::Float, math::Tanh::zero());
        float_fn_table.table.push(TypeV::Float, math::ASinh::zero());
        float_fn_table.table.push(TypeV::Float, math::ACosh::zero());
        float_fn_table.table.push(TypeV::Float, math::ATanh::zero());
        float_fn_table.table.push(TypeV::Float, math::Exp::zero());
        float_fn_table.table.push(TypeV::Float, math::Log::zero());
        float_fn_table.table.push(TypeV::Float, math::Abs::zero());
        float_fn_table.table.push(TypeV::Float, math::Heaviside::zero());
        float_fn_table.table.push(TypeV::Float, math::ReLu::zero());
        float_fn_table.table.push(
            TypeV::Float,
            ops::Add::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        float_fn_table.table.push(
            TypeV::Float,
            ops::Sub::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        float_fn_table.table.push(
            TypeV::Float,
            ops::Mul::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        float_fn_table.table.push(
            TypeV::Float,
            ops::Div::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );
        float_fn_table.table.push(
            TypeV::Float,
            ops::Pow::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
        );

        float_fn_table
    }
}
