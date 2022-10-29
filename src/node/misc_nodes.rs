use super::base::*;
pub struct Cond {
    pub rtype: TypeV,
    pub arg_types: Vec<TypeV>,
    pub cond: NodeRef,
    pub iftrue: NodeRef,
    pub iffalse: NodeRef,
}

impl Cond {
    pub fn new(cond: NodeRef, iftrue: NodeRef, iffalse: NodeRef) -> NodeRef {
        let rtype = iftrue.get_rtype();
        assert_eq!(iftrue.get_rtype(), iffalse.get_rtype());
        assert_eq!(cond.get_rtype(), TypeV::Bool);
        Box::new(Cond {
            rtype: rtype,
            arg_types: vec![cond.get_rtype(), iftrue.get_rtype(), iffalse.get_rtype()],
            cond: cond,
            iftrue: iftrue,
            iffalse: iffalse,
        })
    }
    pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
        Box::new(Cond {
            rtype: rtype,
            arg_types: arg_types,
            cond: Null::zero(rtype),
            iftrue: Null::zero(rtype),
            iffalse: Null::zero(rtype),
        })
    }
}
impl Node for Cond {
    fn to_str(&self, indent: usize) -> String {
        " ".repeat(indent)
            + "?\n"
            + &self.cond.to_str(indent + 1)
            + "\n"
            + &self.iftrue.to_str(indent + 1)
            + &self.iffalse.to_str(indent + 1)
    }
    fn eval(&self, args: &[Type]) -> Type {
        let c = self.cond.eval(args);
        let t = self.iftrue.eval(args);
        let f = self.iffalse.eval(args);
        match (c, t, f) {
            (Type::Bool(ci), Type::Float(ti), Type::Float(fi)) => if ci { Type::Float(ti) } else {Type::Float(fi)},
            (Type::Bool(ci), Type::Int(ti), Type::Int(fi)) => if ci { Type::Int(ti) } else {Type::Int(fi)},
            (Type::Bool(ci), Type::UInt(ti), Type::UInt(fi)) => if ci { Type::UInt(ti) } else {Type::UInt(fi)},
            _ => panic!("Invalid: Cond must be bool (Got {:?}) Branches must be of same type (Got {:?} and {:?})", c, t, f)
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
            0 => self.cond = child,
            1 => self.iftrue = child,
            2 => self.iffalse = child,
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
        let cond = build_table
            .get_rand_node(depth + 1, TypeV::Bool, params)
            .build_random_node(build_table, arg_types, TypeV::Bool, depth + 1, params);
        let iftrue = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        let iffalse = build_table
            .get_rand_node(depth + 1, node_rtype, params)
            .build_random_node(build_table, arg_types, node_rtype, depth + 1, params);
        node.set_child(0, cond);
        node.set_child(1, iftrue);
        node.set_child(1, iffalse);
        node
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        if (self.cond.get_rtype() == TypeV::Bool)
            && (self.iftrue.get_rtype() == self.rtype)
            && (self.iffalse.get_rtype() == self.rtype)
        {
            if let Err(err) = self.cond.type_check() {
                return Err(err);
            } else {
                if let Err(err) = self.iftrue.type_check() {
                    return Err(err);
                } else {
                    if let Err(err) = self.iffalse.type_check() {
                        return Err(err);
                    } else {
                        return Ok(());
                    }
                }
            }
        } else {
            return Err(TypeErr {msg: format!("Cond required argument of type ({:#?}, {:#?}, {:#?}); Got ({:#?}, {:#?}, {:#?})!!", TypeV::Bool, self.rtype, self.rtype, self.cond.get_rtype(), self.iftrue.get_rtype(), self.iffalse.get_rtype())});
        }
    }
}
