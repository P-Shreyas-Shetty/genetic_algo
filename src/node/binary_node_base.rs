use std::marker::PhantomData;

use super::base::*;
use rand::Rng;

/// For now I am not using this macro because it is a little hard to use
#[macro_export]
macro_rules! impl_binary_operation_node {
    (name => $name: ident, str_ident => $str_ident:tt, expression => ($lhs: ident, $rhs:ident) -> $expression:expr) => {
        pub struct $name {
            pub rtype: TypeV,
            pub arg_types: Vec<TypeV>,
            pub rhs: NodeRef,
            pub lhs: NodeRef,
        }

        impl $name {
            pub fn make(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
                let rtype = rhs.get_rtype();
                assert_eq!(rhs.get_rtype(), lhs.get_rtype());
                Box::new(Self {
                    rtype,
                    arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
                    rhs,
                    lhs,
                })
            }
            pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
                Box::new(Self {
                    rtype,
                    arg_types,
                    rhs: Null::zero(rtype),
                    lhs: Null::zero(rtype),
                })
            }
        }

        impl Node for $name {
            fn to_str(&self, indent: usize) -> String {
                ".".repeat(indent)
                    + $str_ident
                    + "\n"
                    + &self.lhs.to_str(indent + 1)
                    + "\n"
                    + &self.rhs.to_str(indent + 1)
            }
            fn eval(&self, args: &[Type]) -> Type {
                let $rhs = self.rhs.eval(args);
                let $lhs = self.lhs.eval(args);
                $expression
            }
            fn get_rtype(&self) -> TypeV {
                self.rtype
            }
            fn get_arg_types(&self) -> &[TypeV] {
                &self.arg_types
            }

            fn set_child(&mut self, child_index: usize, child: NodeRef) {
                match child_index {
                    0 => self.lhs = child,
                    1 => self.rhs = child,
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
                let lhs = build_table
                    .get_rand_node(depth + 1, node_rtype, params)
                    .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
                let rhs = build_table
                    .get_rand_node(depth + 1, node_rtype, params)
                    .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
                node.set_child(0, lhs);
                node.set_child(1, rhs);
                node
            }
            fn type_check(&self) -> Result<(), TypeErr> {
                if (self.lhs.get_rtype() == self.arg_types[0])
                    && (self.rhs.get_rtype() == self.arg_types[1])
                {
                    self.lhs.type_check()?;
                    self.rhs.type_check()?;
                    Ok(())
                } else {
                    Err(TypeErr {
                        msg: format!(
                            "{} required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                            stringify!($name),
                            self.arg_types[0],
                            self.arg_types[1],
                            self.lhs.get_rtype(),
                            self.rhs.get_rtype()
                        ),
                    })
                }
            }
            fn deep_copy(&self) -> NodeRef {
                Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
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
                    let lhs = self.lhs.mutant_copy(
                        probability,
                        node_depth + 1,
                        arg_types,
                        build_table,
                        params,
                    );
                    let rhs = self.rhs.mutant_copy(
                        probability,
                        node_depth + 1,
                        arg_types,
                        build_table,
                        params,
                    );
                    match (lhs, rhs) {
                        (None, None) => None, //If both child nodes' mutation was unsuccessful, then this node wasn't mutated. So return None
                        (lhs, rhs) => {
                            //If either of children mutated, then node is mutated. Copy the node that wasn't mutated
                            let mut ret = Self::zero(self.rtype, self.arg_types.clone());
                            let lhs_s = if let Some(lhs_s) = lhs {
                                lhs_s
                            } else {
                                self.lhs.deep_copy()
                            };
                            let rhs_s = if let Some(rhs_s) = rhs {
                                rhs_s
                            } else {
                                self.rhs.deep_copy()
                            };
                            ret.set_child(0, lhs_s);
                            ret.set_child(1, rhs_s);
                            Some(ret)
                        }
                    }
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
                    let maybe_rhs = self.rhs.get_random_child(probability, depth + 1, params);
                    let maybe_lhs = self.lhs.get_random_child(probability, depth + 1, params);

                    match (maybe_lhs, maybe_rhs) {
                        (maybe_lhs, None) => maybe_lhs,
                        (None, maybe_rhs) => maybe_rhs,
                        (mlhs, mrhs) => {
                            if params.randomizer.gen::<f32>() < 0.5 {
                                mlhs
                            } else {
                                mrhs
                            }
                        }
                    }
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
                    if new_node.get_rtype() == self.get_rtype() {
                        Some(new_node)
                    } else {
                        None
                    }
                } else {
                    #[allow(clippy::collapsible_else_if)]
                    if params.randomizer.gen::<f32>() < 0.5 {
                        let lhs =
                            self.lhs
                                .set_random_child(new_node, probability, depth + 1, params)?;
                        let rhs = self.rhs.deep_copy();
                        Some(Self::make(rhs, lhs))
                    } else {
                        let rhs =
                            self.rhs
                                .set_random_child(new_node, probability, depth + 1, params)?;
                        let lhs = self.lhs.deep_copy();
                        Some(Self::make(rhs, lhs))
                    }
                }
            }
        }
    };
}

/// This trait is used to better reuse for binary operations
pub trait BinOpKind {
    /// This is the string representation of the operator
    const NAME: &'static str;

    /// eval function is defined for the operation
    fn eval(lhs: Type, rhs: Type) -> Type;
}

/// BinOpBase<KIND> is used to specialize for operation
/// Ex: type Add = BinOpBase<AddEval>;
pub struct BinOpBase<T: BinOpKind> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
    phantom: PhantomData<T>,
}

impl<T: 'static + BinOpKind> BinOpBase<T> {
    pub fn make(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Self {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
            phantom: PhantomData::default(),
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Self {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
            phantom: PhantomData::default(),
        })
    }
}

impl<T: 'static + BinOpKind> Node for BinOpBase<T> {
    fn get_tree_str(&self, indent: usize) -> String {
        ".".repeat(indent)
            + T::NAME
            + "\n"
            + &self.lhs.get_tree_str(indent + 1)
            + "\n"
            + &self.rhs.get_tree_str(indent + 1)
    }
    ///returns equation in string format
    fn get_equation_str(&self)->String {
        format!("({} {} {} )", self.lhs.get_equation_str(), T::NAME, self.rhs.get_equation_str())
    }
    fn eval(&self, args: &[Type]) -> Type {
        let rhs = self.rhs.eval(args);
        let lhs = self.lhs.eval(args);
        T::eval(lhs, rhs)
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }

    fn set_child(&mut self, child_index: usize, child: NodeRef) {
        match child_index {
            0 => self.lhs = child,
            1 => self.rhs = child,
            _ => unreachable!(),
        }
    }

    fn get_child(&self, child_index: usize) -> &NodeRef {
        match child_index {
            0 => &self.lhs,
            1 => &self.rhs,
            _ => unreachable!(),
        }
    }

    fn get_zero_node(&self) -> NodeRef {
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
        let mut node = Self::get_zero_node(self);
        let lhs = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        let rhs = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        node.set_child(0, lhs);
        node.set_child(1, rhs);
        node
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        if (self.lhs.get_rtype() == self.arg_types[0])
            && (self.rhs.get_rtype() == self.arg_types[1])
        {
            self.lhs.type_check()?;
            self.rhs.type_check()?;
            Ok(())
        } else {
            Err(TypeErr {
                msg: format!(
                    "{} required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    stringify!($name),
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef {
        Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
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
            let lhs =
                self.lhs
                    .mutant_copy(probability, node_depth + 1, arg_types, build_table, params);
            let rhs =
                self.rhs
                    .mutant_copy(probability, node_depth + 1, arg_types, build_table, params);
            match (lhs, rhs) {
                (None, None) => None, //If both child nodes' mutation was unsuccessful, then this node wasn't mutated. So return None
                (lhs, rhs) => {
                    //If either of children mutated, then node is mutated. Copy the node that wasn't mutated
                    let mut ret = Self::zero(self.rtype, self.arg_types.clone());
                    let lhs_s = if let Some(lhs_s) = lhs {
                        lhs_s
                    } else {
                        self.lhs.deep_copy()
                    };
                    let rhs_s = if let Some(rhs_s) = rhs {
                        rhs_s
                    } else {
                        self.rhs.deep_copy()
                    };
                    ret.set_child(0, lhs_s);
                    ret.set_child(1, rhs_s);
                    Some(ret)
                }
            }
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
            let maybe_rhs = self.rhs.get_random_child(probability, depth + 1, params);
            let maybe_lhs = self.lhs.get_random_child(probability, depth + 1, params);

            match (maybe_lhs, maybe_rhs) {
                (maybe_lhs, None) => maybe_lhs,
                (None, maybe_rhs) => maybe_rhs,
                (mlhs, mrhs) => {
                    if params.randomizer.gen::<f32>() < 0.5 {
                        mlhs
                    } else {
                        mrhs
                    }
                }
            }
        }
    }

    fn set_random_child(
        &self,
        new_node: NodeRef,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        if depth + new_node.get_max_depth() > params.max_depth {
            None
        } else if new_node.get_rtype() == self.get_rtype()
            && params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth)
        {
            if new_node.get_rtype() == self.get_rtype() {
                Some(new_node)
            } else {
                None
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if params.randomizer.gen::<f32>() < 0.5 {
                let lhs = self
                    .lhs
                    .set_random_child(new_node, probability, depth + 1, params)?;
                let rhs = self.rhs.deep_copy();
                Some(Self::make(rhs, lhs))
            } else {
                let rhs = self
                    .rhs
                    .set_random_child(new_node, probability, depth + 1, params)?;
                let lhs = self.lhs.deep_copy();
                Some(Self::make(rhs, lhs))
            }
        }
    }

    fn prune(&self) -> NodeRef {
        //This does nothing because even in case where there are wasteful slots in
        //binary nodes, those will require more complex analysis with specific cases
        Self::make(self.rhs.prune(), self.lhs.prune())
    }

    fn get_name(&self) -> &'static str {
        T::NAME
    }

    fn get_max_depth(&self) -> usize {
        usize::max(self.lhs.get_max_depth(), self.rhs.get_max_depth()) + 1
    }
}
