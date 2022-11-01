#![allow(dead_code)]
use super::base::*;
use rand::Rng;

pub struct And {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
}

impl And {
    pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        assert_eq!(rhs.get_rtype(), TypeV::Bool);
        Box::new(And {
            rtype: rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs: rhs,
            lhs: lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(And {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl Node for And {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "&\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(ri & li),
            _ => panic!("Invalid: Can't and {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        return &self.arg_types;
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
        if (self.lhs.get_rtype() == TypeV::Bool) && (self.rhs.get_rtype() == TypeV::Bool) {
            if let Err(err) = self.lhs.type_check() {
                return Err(err);
            } else {
                if let Err(err) = self.rhs.type_check() {
                    return Err(err);
                } else {
                    return Ok(());
                }
            }
        } else {
            return Err(TypeErr {
                msg: format!(
                    "And required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    TypeV::Bool,
                    TypeV::Bool,
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            });
        }
    }
    fn deep_copy(&self) -> NodeRef {
        Self::new(self.rhs.deep_copy(), self.lhs.deep_copy())
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
                    return Some(ret);
                }
            }
        }
    }
}

pub struct Or {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
}

impl Or {
    pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        assert_eq!(rhs.get_rtype(), TypeV::Bool);
        Box::new(Or {
            rtype: rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs: rhs,
            lhs: lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Or {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
    fn deep_copy(&self) -> NodeRef {
        Self::new(self.rhs.deep_copy(), self.lhs.deep_copy())
    }
}

impl Node for Or {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "|\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(ri | li),
            _ => panic!("Invalid: Can't Or {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        return &self.arg_types;
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
        if (self.lhs.get_rtype() == TypeV::Bool) && (self.rhs.get_rtype() == TypeV::Bool) {
            if let Err(err) = self.lhs.type_check() {
                return Err(err);
            } else {
                if let Err(err) = self.rhs.type_check() {
                    return Err(err);
                } else {
                    return Ok(());
                }
            }
        } else {
            return Err(TypeErr {
                msg: format!(
                    "Or required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    TypeV::Bool,
                    TypeV::Bool,
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            });
        }
    }
    fn deep_copy(&self) -> NodeRef {
        Self::new(self.rhs.deep_copy(), self.lhs.deep_copy())
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
                    return Some(ret);
                }
            }
        }
    }
}

pub struct Not {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
}

impl Not {
    pub fn new(rhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), TypeV::Bool);
        Box::new(Not {
            rtype: rtype,
            arg_types: vec![rhs.get_rtype()],
            rhs: rhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Not {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
        })
    }
}

impl Node for Not {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent) + "!\n" + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
        let r = self.rhs.eval(args);
        match r {
            Type::Bool(ri) => Type::Bool(!ri),
            _ => panic!("Invalid: Can't Compl {:?} ", r),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        return &self.arg_types;
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef) {
        match child_index {
            0 => self.rhs = child,
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
        let rhs = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        node.set_child(0, rhs);
        node
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        if self.rhs.get_rtype() == TypeV::Bool {
            if let Err(err) = self.rhs.type_check() {
                return Err(err);
            } else {
                if let Err(err) = self.rhs.type_check() {
                    return Err(err);
                } else {
                    return Ok(());
                }
            }
        } else {
            return Err(TypeErr {
                msg: format!(
                    "Not required argument of type ({:#?}); Got ({:#?})!!",
                    TypeV::Bool,
                    self.rhs.get_rtype()
                ),
            });
        }
    }
    fn deep_copy(&self) -> NodeRef {
        Self::new(self.rhs.deep_copy())
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
            let mut ret = Self::zero(self.rtype, self.arg_types.clone());
            let rhs = self.rhs.mutant_copy(
                probability,
                node_depth + 1,
                arg_types,
                build_table,
                params,
            )?; //If mutation return None, then return None for parent node's mutation
            ret.set_child(1, rhs);
            return Some(ret);
        }
    }
}
