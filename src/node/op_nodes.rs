use super::base::*;
use rand::Rng;
use num::{Integer, Unsigned, Float};

pub struct Add<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Add<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Add {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Add {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Add<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "+\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(ri + li),
            (Type::Float(ri), Type::Float(li)) => Type::Float(ri + li),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(ri + li),
            _ => panic!("Invalid: Can't add {:?} with {:?}", r, l),
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
                    "Add required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
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

pub struct Sub<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Sub<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Sub {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Sub {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Sub<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "-\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(li - ri),
            (Type::Float(ri), Type::Float(li)) => Type::Float(li - ri),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(li - ri),
            _ => panic!("Invalid: Can't Sub {:?} with {:?}", l, r),
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
                    "Sub required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
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

pub struct Mul<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Mul<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Mul {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Mul {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Mul<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "*\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(ri * li),
            (Type::Float(ri), Type::Float(li)) => Type::Float(ri * li),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(ri * li),
            _ => panic!("Invalid: Can't Mul {:?} with {:?}", l, r),
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
                    "Mul required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
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

pub struct Div<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Div<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Div {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Div {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::zero(self.rtype, self.arg_types.clone())
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Div<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "/\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Int(ri), Type::Int(li)) => Type::Int(li / ri),
            (Type::Float(ri), Type::Float(li)) => Type::Float(li / ri),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(li / ri),
            _ => panic!("Invalid: Can't Div {:?} with {:?}", l, r),
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
                    "Div required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
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
pub struct Pow<F: Float, I: Integer, U: Unsigned> {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef<F,I,U>,
    pub lhs: NodeRef<F,I,U>,
}

impl<F: Float, I: Integer, U: Unsigned> Pow<F,I,U> {
    pub fn make(rhs: NodeRef<F,I,U>, lhs: NodeRef<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Pow {
            rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs,
            lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef<F,I,U> {
        Box::new(Pow {
            rtype,
            arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl<F: Float, I: Integer, U: Unsigned> Node<F,I,U> for Pow<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "**\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Int(ri), Type::Int(li)) => Type::Float(f32::powf(li as f32, ri as f32)),
            (Type::UInt(ri), Type::Int(li)) => Type::Int(i32::pow(li, ri as u32)),
            (Type::Float(ri), Type::Float(li)) => Type::Float(f32::powf(li as f32, ri as f32)),
            (Type::Int(ri), Type::Float(li)) => Type::Float(f32::powf(li as f32, ri as f32)),
            (Type::UInt(ri), Type::Float(li)) => Type::Float(f32::powf(li as f32, ri as f32)),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(u32::pow(li, ri as u32)),
            _ => panic!("Invalid: Can't Exp {:?} with {:?}", r, l),
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
                    "Pow required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
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
