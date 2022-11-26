use super::base::*;
use super::unary_node_base as ub;

macro_rules! single_arg_fn_node_def {
    ($type_name: ident, $temp_type_name: ident, $expr_fn: expr) => {
        pub struct $temp_type_name {}

        impl ub::UnaryOpKind for $temp_type_name {
            const NAME: &'static str = stringify!($type_name);
            const ARG_TYPE: TypeV = TypeV::Float;
            const RTYPE: TypeV = TypeV::Float;

            fn eval(input: Type) -> Type {
                if let Type::Float(f) = input {
                    Type::Float($expr_fn(f))
                } else {
                    unreachable!();
                }
            }
        }

        pub type $type_name = ub::UnaryOpBase<$temp_type_name>;
    };
}

single_arg_fn_node_def!(Sin, __Sin, f32::sin);
single_arg_fn_node_def!(Cos, __Cos, f32::cos);
single_arg_fn_node_def!(Tan, __Tan, f32::tan);
single_arg_fn_node_def!(ASin, __ASin, f32::asin);
single_arg_fn_node_def!(ACos, __ACos, f32::acos);
single_arg_fn_node_def!(ATan, __ATan, f32::atan);
single_arg_fn_node_def!(Sinh, __Sinh, f32::sinh);
single_arg_fn_node_def!(Cosh, __Cosh, f32::cosh);
single_arg_fn_node_def!(Tanh, __Tanh, f32::tanh);
single_arg_fn_node_def!(ASinh, __ASinh, f32::asinh);
single_arg_fn_node_def!(ACosh, __ACosh, f32::acosh);
single_arg_fn_node_def!(ATanh, __ATanh, f32::atanh);
single_arg_fn_node_def!(Exp, __Exp, f32::exp);
single_arg_fn_node_def!(Log, __Log, f32::ln);
single_arg_fn_node_def!(Abs, __Abs, f32::abs);

pub struct __Heaviside {}

impl ub::UnaryOpKind for __Heaviside {
    const NAME:&'static str = "Heaviside";
    const ARG_TYPE: TypeV = TypeV::Float;
    const RTYPE: TypeV = TypeV::Float;
    
    fn eval(input: Type) -> Type {
        match input {
            Type::Float(x) => {
                if x>=0.0 {Type::Float(1.0)} else {Type::Float(0.0)}
            }
            _ => unreachable!()
        }
    }
}

#[allow(dead_code)]
/// Heaviside function; defined as 
/// H(x) = (x>=0) ? 1.0; 0.0
pub type Heaviside = ub::UnaryOpBase<__Heaviside>;

pub struct __ReLu {}

impl ub::UnaryOpKind for __ReLu {
    const NAME:&'static str = "ReLu";
    const ARG_TYPE: TypeV = TypeV::Float;
    const RTYPE: TypeV = TypeV::Float;
    
    fn eval(input: Type) -> Type {
        match input {
            Type::Float(x) => {
                if x>=0.0 {Type::Float(x)} else {Type::Float(0.0)}
            }
            _ => unreachable!()
        }
    }
}

#[allow(dead_code)]
/// ReLu function; defined as 
/// ReLu(x) = (x>=0) ? x; 0.0
pub type ReLu = ub::UnaryOpBase<__ReLu>;