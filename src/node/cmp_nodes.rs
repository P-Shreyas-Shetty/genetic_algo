use super::base::*;
use num::{Float, Integer, Unsigned};
use rand::Rng;


pub struct Eq<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F, I, U>,
    pub lhs: NodeRef<F, I, U>,
}

impl<F: Float, I: Integer, U: Unsigned> Eq<F, I, U> {
    pub fn make(rhs: NodeRef<F, I, U>, lhs: NodeRef<F, I, U>) -> NodeRef<F, I, U> {
        let rtype = TypeV::Bool;
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Eq {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F, I, U> {
        Box::new(Eq {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F, I, U> for Eq<F, I, U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "==\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F, I, U>]) -> Type<F, I, U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri == li),
            (Type::Float(ri), Type::Int(li)) => Type::Bool(ri == li as f32),
            (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri == li as f32),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri == li),
            (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri == li as i32),
            (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f32 == li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri == li),
            (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i32 == li),
            (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f32 == li),
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(ri == li),
            _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef<F, I, U>) {
        match child_index {
            0 => self.lhs = child,
            1 => self.rhs = child,
            _ => unreachable!(),
        }
    }
    fn get_type_zero(&self) -> NodeRef<F, I, U> {
        Self::zero(self.rtype, self.arg_types.clone())
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable<F, I, U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F, I, U> {
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
                    "Eq required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef<F, I, U> {
        Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable<F, I, U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F, I, U>> {
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
}

pub struct NEq<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> NEq<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = TypeV::Bool;
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(NEq {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(NEq {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for NEq<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "!=\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri != li),
            (Type::Float(ri), Type::Int(li)) => Type::Bool(ri != li as f32),
            (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri != li as f32),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri != li),
            (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri != li as i32),
            (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f32 != li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri != li),
            (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i32 != li),
            (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f32 != li),
            (Type::Bool(ri), Type::Bool(li)) => Type::Bool(ri != li),
            _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef<F,I,U>) {
        match child_index {
            0 => self.lhs = child,
            1 => self.rhs = child,
            _ => unreachable!(),
        }
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::zero(self.rtype, self.arg_types.clone())
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable<F,I,U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
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
                    "NEq required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
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
}

pub struct Gt<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Gt<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = TypeV::Bool;
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Gt {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef <F,I,U>{
        Box::new(Gt {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Gt<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + ">\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri > li),
            (Type::Float(ri), Type::Int(li)) => Type::Bool(ri > li as f32),
            (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri > li as f32),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri > li),
            (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri > li as i32),
            (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f32 > li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri > li),
            (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i32 > li),
            (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f32 > li),
            _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef<F,I,U>) {
        match child_index {
            0 => self.lhs = child,
            1 => self.rhs = child,
            _ => unreachable!(),
        }
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::zero(self.rtype, self.arg_types.clone())
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable<F,I,U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
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
                    "Gt required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
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
}

pub struct Gte<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Gte<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = TypeV::Bool;
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Gte {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Gte {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Gte<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + ">=\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri >= li),
            (Type::Float(ri), Type::Int(li)) => Type::Bool(ri >= li as f32),
            (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri >= li as f32),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri >= li),
            (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri >= li as i32),
            (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f32 >= li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri >= li),
            (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i32 >= li),
            (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f32 >= li),
            _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef<F,I,U>) {
        match child_index {
            0 => self.lhs = child,
            1 => self.rhs = child,
            _ => unreachable!(),
        }
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::zero(self.rtype, self.arg_types.clone())
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable<F,I,U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
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
                    "Gte required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
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
}

pub struct Lt<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Lt<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = TypeV::Bool;
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Lt {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Lt {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Lt<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "<\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri < li),
            (Type::Float(ri), Type::Int(li)) => Type::Bool(ri < li as f32),
            (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri < li as f32),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri < li),
            (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri < li as i32),
            (Type::Int(ri), Type::Float(li)) => Type::Bool((ri as f32) < li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri < li),
            (Type::UInt(ri), Type::Int(li)) => Type::Bool((ri as i32) < li),
            (Type::UInt(ri), Type::Float(li)) => Type::Bool((ri as f32) < li),
            _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef<F,I,U>) {
        match child_index {
            0 => self.lhs = child,
            1 => self.rhs = child,
            _ => unreachable!(),
        }
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::zero(self.rtype, self.arg_types.clone())
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable<F,I,U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
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
                    "Lt required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
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
}

pub struct Lte<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Lte<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = TypeV::Bool;
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Lte {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Lte {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Lte<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "<=\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Float(ri), Type::Float(li)) => Type::Bool(ri <= li),
            (Type::Float(ri), Type::Int(li)) => Type::Bool(ri <= li as f32),
            (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri <= li as f32),
            (Type::Int(ri), Type::Int(li)) => Type::Bool(ri <= li),
            (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri <= li as i32),
            (Type::Int(ri), Type::Float(li)) => Type::Bool((ri as f32) <= li),
            (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri <= li),
            (Type::UInt(ri), Type::Int(li)) => Type::Bool((ri as i32) <= li),
            (Type::UInt(ri), Type::Float(li)) => Type::Bool((ri as f32) <= li),
            _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
        }
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, child_index: usize, child: NodeRef<F,I,U>) {
        match child_index {
            0 => self.lhs = child,
            1 => self.rhs = child,
            _ => unreachable!(),
        }
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::zero(self.rtype, self.arg_types.clone())
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable<F,I,U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
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
                    "Lte required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            })
        }
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Self::make(self.rhs.deep_copy(), self.lhs.deep_copy())
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
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
}
