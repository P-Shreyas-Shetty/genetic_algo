use super::base::*;
use rand::Rng;

macro_rules! single_arg_fn_node_def {
    ($type_name: ident, $expr_fn: expr) => {
        pub struct $type_name {
            arg: NodeRef,
            arg_types: Vec<TypeV>,
        }

        impl $type_name {
            pub fn zero() -> NodeRef {
                Box::new($type_name {
                    arg: Null::zero(TypeV::Float),
                    arg_types: vec![TypeV::Float],
                })
            }
        }
        impl Node for $type_name {
            fn eval(&self, args: &[Type]) -> Type {
                if let Type::Float(a) = self.arg.eval(args) {
                    return Type::Float($expr_fn(a));
                } else {
                    panic!("Expecetd float as argument for the node {}", self.to_str(0));
                }
            }

            fn get_rtype(&self) -> TypeV {
                return TypeV::Float;
            }

            fn to_str(&self, indent: usize) -> String {
                " ".repeat(indent) + stringify!($type_name) + "\n" + &self.arg.to_str(indent + 1)
            }

            fn get_arg_types(&self) -> &[TypeV] {
                return &self.arg_types;
            }
            fn set_child(&mut self, child_index: usize, child: NodeRef) {
                match child_index {
                    0 => self.arg = child,
                    _ => unreachable!(),
                }
            }
            fn get_type_zero(&self) -> NodeRef {
                Self::zero()
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
                let arg = build_table
                    .get_rand_node(depth + 1, node_rtype, params)
                    .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
                node.set_child(0, arg);
                node
            }
            fn deep_copy(&self) -> NodeRef {
                let mut ret = Self::zero();
                ret.set_child(0, self.arg.deep_copy());
                return ret;
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
                    let arg = self.arg.mutant_copy(
                        probability,
                        node_depth + 1,
                        arg_types,
                        build_table,
                        params,
                    )?; //if child node's mutation was unsuccesful, then this node's mutation was unsuccesful as a whole
                    let mut ret = Self::zero();
                    ret.set_child(0, arg);
                    return Some(ret);
                }
            }
            fn type_check(&self) -> Result<(), TypeErr> {
                if self.arg.get_rtype() == TypeV::Float {
                    return self.arg.type_check();
                } else {
                    return Err(TypeErr {
                        msg: format!(
                            "{} required argument of type {:#?}; Got {:#?}!!",
                            stringify!($type_name),
                            TypeV::Float,
                            self.arg.get_rtype()
                        ),
                    });
                }
            }
        }

        impl FnNode for $type_name {
            fn set_args(&mut self, mut args: Vec<NodeRef>) {
                if let Some(a0) = args.pop() {
                    self.arg = a0;
                }
            }
        }
    };
}

single_arg_fn_node_def!(Sin, f32::sin);
single_arg_fn_node_def!(Cos, f32::cos);
single_arg_fn_node_def!(Tan, f32::tan);
single_arg_fn_node_def!(ASin, f32::asin);
single_arg_fn_node_def!(ACos, f32::acos);
single_arg_fn_node_def!(ATan, f32::atan);
single_arg_fn_node_def!(Sinh, f32::sinh);
single_arg_fn_node_def!(Cosh, f32::cosh);
single_arg_fn_node_def!(Tanh, f32::tanh);
single_arg_fn_node_def!(ASinh, f32::asinh);
single_arg_fn_node_def!(ACosh, f32::acosh);
single_arg_fn_node_def!(ATanh, f32::atanh);
single_arg_fn_node_def!(Exp, f32::exp);
single_arg_fn_node_def!(Log, f32::ln);
single_arg_fn_node_def!(Abs, f32::abs);
