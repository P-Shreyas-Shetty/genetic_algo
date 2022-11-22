use super::base::*;
use rand::Rng;
use std::marker::PhantomData;

/// Most of the methods on unary nodes look alike,
/// so I am making basic nodes for Unary nodes
pub trait UnaryOpKind {
    const NAME: &'static str;
    const ARG_TYPE: TypeV;
    const RTYPE: TypeV;
    fn eval(input: Type) -> Type;
}

/// UnaryOpKind<Op> is used to specialize for
/// individual Unary operations
pub struct UnaryOpBase<T: UnaryOpKind> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub arg: NodeRef,
    phantom: PhantomData<T>,
}

impl<T: 'static + UnaryOpKind> UnaryOpBase<T> {
    pub fn make(child: NodeRef) -> NodeRef {
        Box::new(Self {
            rtype: T::RTYPE,
            arg_types: vec![T::ARG_TYPE],
            arg: child,
            phantom: PhantomData::default(),
        })
    }
    pub fn zero() -> NodeRef {
        Box::new(Self {
            arg: Null::zero(T::RTYPE),
            arg_types: vec![T::ARG_TYPE],
            rtype: T::RTYPE,
            phantom: PhantomData::default(),
        })
    }
}

impl<T: 'static + UnaryOpKind> Node for UnaryOpBase<T> {
    fn eval(&self, args: &[Type]) -> Type {
        let arg_val = self.arg.eval(args);
        T::eval(arg_val)
    }

    fn get_rtype(&self) -> TypeV {
        T::RTYPE
    }

    fn to_str(&self, indent: usize) -> String {
        ".".repeat(indent) + T::NAME + "\n" + &self.arg.to_str(indent + 1)
    }
    ///returns equation in string format
    fn get_equation(&self)->String {
        format!("{}({})", T::NAME, self.arg.get_equation())
    }

    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef) {
        match child_index {
            0 => self.arg = child,
            _ => unreachable!(),
        }
    }
    fn get_child(&self, child_index: usize) -> &NodeRef {
        match child_index {
            0 => &self.arg,
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
        let mut ret = self.get_type_zero();
        ret.set_child(0, self.arg.deep_copy());
        ret
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
            Some(Self::make(arg))
        }
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        if self.arg.get_rtype() == T::RTYPE {
            self.arg.type_check()
        } else {
            Err(TypeErr {
                msg: format!(
                    "{} required argument of type {:#?}; Got {:#?}!!",
                    T::NAME,
                    T::RTYPE,
                    self.arg.get_rtype()
                ),
            })
        }
    }
    fn get_random_child(
        &self,
        probability: f32,
        depth: usize,
        params: &mut BuilderParams,
    ) -> Option<NodeRef> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth) {
            Some(self.deep_copy())
        } else {
            self.arg.get_random_child(probability, depth + 1, params)
        }
    }

    fn set_random_child(
        &self,
        new_node: NodeRef,
        probability: f32,
        depth: usize,
        params: &mut BuilderParams,
    ) -> Option<NodeRef> {
        if depth + new_node.get_max_depth() > params.max_depth {
            None
        } else if new_node.get_rtype() == T::RTYPE
            && params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth)
        {
            Some(new_node)
        } else {
            let child = self
                .arg
                .set_random_child(new_node, probability, depth + 1, params)?;
            Some(Self::make(child))
        }
    }

    fn get_name(&self) -> &'static str {
        T::NAME
    }

    /// This is probably an unsatisfactory way to do it, I'll do better when I find a way
    /// to put this in main class
    fn prune(&self) -> NodeRef {
        match (T::NAME, self.arg.get_name()) {
            ("Sin", "ASin")
            | ("ASin", "Sin")
            | ("Cos", "ACos")
            | ("ACos", "Cos")
            | ("Tan", "ATan")
            | ("ATan", "Tan")
            | ("Sinh", "ASinh")
            | ("ASinh", "Sinh")
            | ("Cosh", "ACosh")
            | ("ACosh", "Cosh")
            | ("Tanh", "ATanh")
            | ("ATanh", "Tanh")
            | ("Log", "Exp")
            | ("Exp", "Log") => self.arg.get_child(0).prune(),
            ("Abs", "Abs") => self.arg.prune(),
            _ => Self::make(self.arg.prune()),
        }
    }

    fn get_max_depth(&self) -> usize {
        self.arg.get_max_depth() + 1
    }
}
