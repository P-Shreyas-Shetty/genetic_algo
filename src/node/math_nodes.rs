use super::base::*;
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

single_arg_fn_node_def!(Sin, f64::sin);
single_arg_fn_node_def!(Cos, f64::cos);
single_arg_fn_node_def!(Tan, f64::tan);
single_arg_fn_node_def!(Exp, f64::exp);
single_arg_fn_node_def!(Log, f64::ln);
single_arg_fn_node_def!(Abs, f64::abs);
