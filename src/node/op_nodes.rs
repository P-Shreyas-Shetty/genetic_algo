use super::base::*;

pub struct Add {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
}

impl Add {
    pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Add {
            rtype: rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs: rhs,
            lhs: lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Add {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl Node for Add {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "+\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
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
        return &&self.arg_types;
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
                    "Add required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            });
        }
    }
}

pub struct Sub {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
}

impl Sub {
    pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Sub {
            rtype: rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs: rhs,
            lhs: lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Sub {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl Node for Sub {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "-\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
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
        if (self.lhs.get_rtype() == self.arg_types[0])
            && (self.rhs.get_rtype() == self.arg_types[1])
        {
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
                    "Sub required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            });
        }
    }
}

pub struct Mul {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
}

impl Mul {
    pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Mul {
            rtype: rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs: rhs,
            lhs: lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Mul {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl Node for Mul {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "*\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
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
        if (self.lhs.get_rtype() == self.arg_types[0])
            && (self.rhs.get_rtype() == self.arg_types[1])
        {
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
                    "Mul required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            });
        }
    }
}

pub struct Div {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
}

impl Div {
    pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Div {
            rtype: rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs: rhs,
            lhs: lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Div {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
    fn get_type_zero(&self) -> NodeRef {
        Self::zero(self.rtype, self.arg_types.clone())
    }
}

impl Node for Div {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "/\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
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
        if (self.lhs.get_rtype() == self.arg_types[0])
            && (self.rhs.get_rtype() == self.arg_types[1])
        {
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
                    "Div required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            });
        }
    }
}
pub struct Pow {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub rhs: NodeRef,
    pub lhs: NodeRef,
}

impl Pow {
    pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
        let rtype = rhs.get_rtype();
        assert_eq!(rhs.get_rtype(), lhs.get_rtype());
        Box::new(Pow {
            rtype: rtype,
            arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
            rhs: rhs,
            lhs: lhs,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Pow {
            rtype: rtype,
            arg_types: arg_types,
            rhs: Null::zero(rtype),
            lhs: Null::zero(rtype),
        })
    }
}

impl Node for Pow {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "**\n"
            + &self.lhs.to_str(indent + 1)
            + "\n"
            + &self.rhs.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
        let r = self.rhs.eval(args);
        let l = self.lhs.eval(args);
        match (r, l) {
            (Type::Int(ri), Type::Int(li)) => Type::Float(f64::powf(li as f64, ri as f64)),
            (Type::UInt(ri), Type::Int(li)) => Type::Int(i64::pow(li, ri as u32)),
            (Type::Float(ri), Type::Float(li)) => Type::Float(f64::powf(li as f64, ri as f64)),
            (Type::Int(ri), Type::Float(li)) => Type::Float(f64::powf(li as f64, ri as f64)),
            (Type::UInt(ri), Type::Float(li)) => Type::Float(f64::powf(li as f64, ri as f64)),
            (Type::UInt(ri), Type::UInt(li)) => Type::UInt(u64::pow(li, ri as u32)),
            _ => panic!("Invalid: Can't Exp {:?} with {:?}", r, l),
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
        if (self.lhs.get_rtype() == self.arg_types[0])
            && (self.rhs.get_rtype() == self.arg_types[1])
        {
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
                    "Pow required argument of type ({:#?}, {:#?}); Got ({:#?}, {:#?})!!",
                    self.arg_types[0],
                    self.arg_types[1],
                    self.lhs.get_rtype(),
                    self.rhs.get_rtype()
                ),
            });
        }
    }
}
