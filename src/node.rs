use rand;
use rand::{thread_rng, Rng};

pub trait Randomize {
    fn random(rtype: TypeV) -> Self;
}

/// Type value
#[derive(std::fmt::Debug, Clone, Copy, std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash)]
pub enum TypeV {
    Int,
    Float,
    UInt,
    Bool,
}

/// Wrapper of Types possible
#[derive(std::fmt::Debug, Clone, Copy)]
pub enum Type {
    Int(i64),
    Float(f64),
    UInt(u64),
    Bool(bool),
}

impl Type {
    //These methods give "Zero" values of the type
    //I intend to use this as a marker type
    //I know its little wasteful, but its fine; I think
    pub fn int(val: i64) -> Type {
        Type::Int(val)
    }
    pub fn float(val: f64) -> Type {
        Type::Float(val)
    }
    pub fn uint(val: u64) -> Type {
        Type::UInt(val)
    }
    pub fn bool(val: bool) -> Type {
        Type::Bool(val)
    }

    pub fn rand(&self) -> Type {
        return match self {
            Type::Int(_) => Type::Int(rand::random()),
            Type::Float(_) => Type::Float(rand::random()),
            Type::UInt(_) => Type::UInt(rand::random()),
            Type::Bool(_) => Type::Bool(rand::random()),
        };
    }

    pub fn int_range(a: i64, b: i64) -> Type {
        let mut rng = thread_rng();
        Type::Int(rng.gen_range(a..=b))
    }
    pub fn float_range(a: f64, b: f64) -> Type {
        let mut rng = thread_rng();
        Type::Float(rng.gen_range(a..=b))
    }

    pub fn uint_range(a: u64, b: u64) -> Type {
        let mut rng = thread_rng();
        Type::UInt(rng.gen_range(a..=b))
    }
    pub fn bool_rand() -> Type {
        Type::Bool(rand::random())
    }
}

impl Randomize for Type {
    fn random(rtype: TypeV) -> Self {
        return match rtype {
            TypeV::Int => Type::Int(rand::random()),
            TypeV::Float => Type::Float(rand::random()),
            TypeV::UInt => Type::UInt(rand::random()),
            TypeV::Bool => Type::Bool(rand::random()),
        };
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, ft: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int(i) => write!(ft, "{}I", i),
            Type::Float(f) => write!(ft, "{}F", f),
            Type::UInt(u) => write!(ft, "{}U", u),
            Type::Bool(b) => write!(ft, "{}", b),
        }
    }
}

pub type NodeRef = Box<dyn Node>;

/// This is the top level Node trait
pub trait Node {
    /// each node is evaluated and value is passed up the tree
    fn eval(&self, args: &[Type]) -> Type;
    fn to_str(&self, indent: usize) -> String;
    fn get_rtype(&self) -> TypeV;
    fn get_arg_types(&self) -> &[TypeV];
    fn get_type_zero(&self) -> NodeRef;
}

/// Special FnNode trait for function node
/// They will define fn_eval function, which'll be used
/// instead of having function member or other mechanisms
pub trait FnNode: Node {
    //fn get_arg_types(&self) -> Vec<TypeV>;
    fn set_args(&mut self, args: Vec<NodeRef>);
}

/// A NUll node. This node does nothing
/// used only for setting "zero" node
pub struct Null {
    rtype: TypeV,
    arg_types: Vec<TypeV>,
}

impl Null {
    pub fn zero(rtype: TypeV) -> NodeRef {
        Box::new(Null {
            rtype,
            arg_types: vec![],
        })
    }
}

impl Node for Null {
    fn to_str(&self, indent: usize) -> String {
        format!("{}{:#?}", " ".repeat(indent), self.rtype)
    }
    fn get_rtype(&self) -> TypeV {
        return self.rtype;
    }
    fn eval(&self, args: &[Type]) -> Type {
        panic!("Cannot evaluate a Null block!!");
    }

    fn get_arg_types(&self) -> &[TypeV] {
        return &self.arg_types;
    }
    fn get_type_zero(&self) -> NodeRef {
        Null::zero(self.rtype)
    }
}

/// Val node for storing constant values
pub struct Val {
    rtype: TypeV,
    pub v: Type,
    arg_types: Vec<TypeV>,
}

impl Val {
    pub fn new(val: Type) -> NodeRef {
        let rtype: TypeV;
        match val {
            Type::Bool(_) => {
                rtype = TypeV::Bool;
            }
            Type::UInt(_) => {
                rtype = TypeV::UInt;
            }
            Type::Int(_) => {
                rtype = TypeV::Int;
            }
            Type::Float(_) => {
                rtype = TypeV::Float;
            }
        }
        Box::new(Val {
            v: val,
            rtype: rtype,
            arg_types: vec![],
        })
    }
    pub fn zero(rtype: TypeV) -> NodeRef {
        let v = match rtype {
            TypeV::Int => Type::int(0),
            TypeV::Float => Type::float(0.0),
            TypeV::UInt => Type::uint(0),
            TypeV::Bool => Type::bool(false),
        };
        return Box::new(Val {
            v,
            rtype,
            arg_types: vec![],
        });
    }
}

impl Node for Val {
    fn to_str(&self, indent: usize) -> String {
        format!("{}{}", " ".repeat(indent), self.v)
    }
    /// On evaluation, value returns constant it represents
    fn eval(&self, _: &[Type]) -> Type {
        return self.v;
    }

    fn get_rtype(&self) -> TypeV {
        self.rtype
    }

    fn get_arg_types(&self) -> &[TypeV] {
        return &self.arg_types;
    }
    fn get_type_zero(&self) -> NodeRef {
        Self::zero(self.rtype)
    }
}

pub struct Var {
    rtype: TypeV,
    pub idx: usize,
    arg_types: Vec<TypeV>,
}

impl Var {
    pub fn new(idx: usize, rtype: TypeV) -> NodeRef {
        Box::new(Var {
            idx: idx,
            rtype: rtype,
            arg_types: vec![],
        })
    }
}

impl Node for Var {
    fn to_str(&self, indent: usize) -> String {
        format!("{}x[{}]", " ".repeat(indent), self.idx)
    }
    fn eval(&self, args: &[Type]) -> Type {
        return args[self.idx];
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        return &self.arg_types;
    }
    fn get_type_zero(&self) -> NodeRef {
        Self::new(0, self.rtype)
    }
}

pub mod ops {
    use super::*;
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }
}
pub mod logic {
    use super::*;
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }
}

pub mod cmp {
    use super::*;
    pub struct Eq {
        pub rtype: TypeV,
        pub arg_types: Vec<TypeV>,
        pub rhs: NodeRef,
        pub lhs: NodeRef,
    }

    impl Eq {
        pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
            let rtype = TypeV::Bool;
            assert_eq!(rhs.get_rtype(), lhs.get_rtype());
            Box::new(Eq {
                rtype: rtype,
                arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
                rhs: rhs,
                lhs: lhs,
            })
        }
        pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
            Box::new(Eq {
                rtype: rtype,
                arg_types: arg_types,
                rhs: Null::zero(rtype),
                lhs: Null::zero(rtype),
            })
        }
    }

    impl Node for Eq {
        fn to_str(&self, indent: usize) -> String {
            " ".repeat(indent)
                + "==\n"
                + &self.lhs.to_str(indent + 1)
                + "\n"
                + &self.rhs.to_str(indent + 1)
        }
        fn eval(&self, args: &[Type]) -> Type {
            let r = self.rhs.eval(args);
            let l = self.lhs.eval(args);
            match (r, l) {
                (Type::Float(ri), Type::Float(li)) => Type::Bool(ri == li),
                (Type::Float(ri), Type::Int(li)) => Type::Bool(ri == li as f64),
                (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri == li as f64),
                (Type::Int(ri), Type::Int(li)) => Type::Bool(ri == li),
                (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri == li as i64),
                (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f64 == li),
                (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri == li),
                (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i64 == li),
                (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f64 == li),
                (Type::Bool(ri), Type::Bool(li)) => Type::Bool(ri == li),
                _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
            }
        }
        fn get_rtype(&self) -> TypeV {
            self.rtype
        }
        fn get_arg_types(&self) -> &[TypeV] {
            return &self.arg_types;
        }
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }

    pub struct NEq {
        pub rtype: TypeV,
        pub arg_types: Vec<TypeV>,
        pub rhs: NodeRef,
        pub lhs: NodeRef,
    }

    impl NEq {
        pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
            let rtype = TypeV::Bool;
            assert_eq!(rhs.get_rtype(), lhs.get_rtype());
            Box::new(NEq {
                rtype: rtype,
                arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
                rhs: rhs,
                lhs: lhs,
            })
        }
        pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
            Box::new(NEq {
                rtype: rtype,
                arg_types: arg_types,
                rhs: Null::zero(rtype),
                lhs: Null::zero(rtype),
            })
        }
    }

    impl Node for NEq {
        fn to_str(&self, indent: usize) -> String {
            " ".repeat(indent)
                + "!=\n"
                + &self.lhs.to_str(indent + 1)
                + "\n"
                + &self.rhs.to_str(indent + 1)
        }
        fn eval(&self, args: &[Type]) -> Type {
            let r = self.rhs.eval(args);
            let l = self.lhs.eval(args);
            match (r, l) {
                (Type::Float(ri), Type::Float(li)) => Type::Bool(ri != li),
                (Type::Float(ri), Type::Int(li)) => Type::Bool(ri != li as f64),
                (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri != li as f64),
                (Type::Int(ri), Type::Int(li)) => Type::Bool(ri != li),
                (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri != li as i64),
                (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f64 != li),
                (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri != li),
                (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i64 != li),
                (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f64 != li),
                (Type::Bool(ri), Type::Bool(li)) => Type::Bool(ri != li),
                _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
            }
        }
        fn get_rtype(&self) -> TypeV {
            self.rtype
        }
        fn get_arg_types(&self) -> &[TypeV] {
            return &self.arg_types;
        }
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }

    pub struct Gt {
        pub rtype: TypeV,
        pub arg_types: Vec<TypeV>,
        pub rhs: NodeRef,
        pub lhs: NodeRef,
    }

    impl Gt {
        pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
            let rtype = TypeV::Bool;
            assert_eq!(rhs.get_rtype(), lhs.get_rtype());
            Box::new(Gt {
                rtype: rtype,
                arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
                rhs: rhs,
                lhs: lhs,
            })
        }
        pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
            Box::new(Gt {
                rtype: rtype,
                arg_types: arg_types,
                rhs: Null::zero(rtype),
                lhs: Null::zero(rtype),
            })
        }
    }

    impl Node for Gt {
        fn to_str(&self, indent: usize) -> String {
            " ".repeat(indent)
                + ">\n"
                + &self.lhs.to_str(indent + 1)
                + "\n"
                + &self.rhs.to_str(indent + 1)
        }
        fn eval(&self, args: &[Type]) -> Type {
            let r = self.rhs.eval(args);
            let l = self.lhs.eval(args);
            match (r, l) {
                (Type::Float(ri), Type::Float(li)) => Type::Bool(ri > li),
                (Type::Float(ri), Type::Int(li)) => Type::Bool(ri > li as f64),
                (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri > li as f64),
                (Type::Int(ri), Type::Int(li)) => Type::Bool(ri > li),
                (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri > li as i64),
                (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f64 > li),
                (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri > li),
                (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i64 > li),
                (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f64 > li),
                _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
            }
        }
        fn get_rtype(&self) -> TypeV {
            self.rtype
        }
        fn get_arg_types(&self) -> &[TypeV] {
            return &self.arg_types;
        }
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }

    pub struct Gte {
        pub rtype: TypeV,
        pub arg_types: Vec<TypeV>,
        pub rhs: NodeRef,
        pub lhs: NodeRef,
    }

    impl Gte {
        pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
            let rtype = TypeV::Bool;
            assert_eq!(rhs.get_rtype(), lhs.get_rtype());
            Box::new(Gte {
                rtype: rtype,
                arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
                rhs: rhs,
                lhs: lhs,
            })
        }
        pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
            Box::new(Gte {
                rtype: rtype,
                arg_types: arg_types,
                rhs: Null::zero(rtype),
                lhs: Null::zero(rtype),
            })
        }
    }

    impl Node for Gte {
        fn to_str(&self, indent: usize) -> String {
            " ".repeat(indent)
                + ">=\n"
                + &self.lhs.to_str(indent + 1)
                + "\n"
                + &self.rhs.to_str(indent + 1)
        }
        fn eval(&self, args: &[Type]) -> Type {
            let r = self.rhs.eval(args);
            let l = self.lhs.eval(args);
            match (r, l) {
                (Type::Float(ri), Type::Float(li)) => Type::Bool(ri >= li),
                (Type::Float(ri), Type::Int(li)) => Type::Bool(ri >= li as f64),
                (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri >= li as f64),
                (Type::Int(ri), Type::Int(li)) => Type::Bool(ri >= li),
                (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri >= li as i64),
                (Type::Int(ri), Type::Float(li)) => Type::Bool(ri as f64 >= li),
                (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri >= li),
                (Type::UInt(ri), Type::Int(li)) => Type::Bool(ri as i64 >= li),
                (Type::UInt(ri), Type::Float(li)) => Type::Bool(ri as f64 >= li),
                _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
            }
        }
        fn get_rtype(&self) -> TypeV {
            self.rtype
        }
        fn get_arg_types(&self) -> &[TypeV] {
            return &self.arg_types;
        }
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }

    pub struct Lt {
        pub rtype: TypeV,
        pub arg_types: Vec<TypeV>,
        pub rhs: NodeRef,
        pub lhs: NodeRef,
    }

    impl Lt {
        pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
            let rtype = TypeV::Bool;
            assert_eq!(rhs.get_rtype(), lhs.get_rtype());
            Box::new(Lt {
                rtype: rtype,
                arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
                rhs: rhs,
                lhs: lhs,
            })
        }
        pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
            Box::new(Lt {
                rtype: rtype,
                arg_types: arg_types,
                rhs: Null::zero(rtype),
                lhs: Null::zero(rtype),
            })
        }
    }

    impl Node for Lt {
        fn to_str(&self, indent: usize) -> String {
            " ".repeat(indent)
                + "<\n"
                + &self.lhs.to_str(indent + 1)
                + "\n"
                + &self.rhs.to_str(indent + 1)
        }
        fn eval(&self, args: &[Type]) -> Type {
            let r = self.rhs.eval(args);
            let l = self.lhs.eval(args);
            match (r, l) {
                (Type::Float(ri), Type::Float(li)) => Type::Bool(ri < li),
                (Type::Float(ri), Type::Int(li)) => Type::Bool(ri < li as f64),
                (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri < li as f64),
                (Type::Int(ri), Type::Int(li)) => Type::Bool(ri < li),
                (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri < li as i64),
                (Type::Int(ri), Type::Float(li)) => Type::Bool((ri as f64) < li),
                (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri < li),
                (Type::UInt(ri), Type::Int(li)) => Type::Bool((ri as i64) < li),
                (Type::UInt(ri), Type::Float(li)) => Type::Bool((ri as f64) < li),
                _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
            }
        }
        fn get_rtype(&self) -> TypeV {
            self.rtype
        }
        fn get_arg_types(&self) -> &[TypeV] {
            return &self.arg_types;
        }
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }

    pub struct Lte {
        pub rtype: TypeV,
        pub arg_types: Vec<TypeV>,
        pub rhs: NodeRef,
        pub lhs: NodeRef,
    }

    impl Lte {
        pub fn new(rhs: NodeRef, lhs: NodeRef) -> NodeRef {
            let rtype = TypeV::Bool;
            assert_eq!(rhs.get_rtype(), lhs.get_rtype());
            Box::new(Lte {
                rtype: rtype,
                arg_types: vec![lhs.get_rtype(), rhs.get_rtype()],
                rhs: rhs,
                lhs: lhs,
            })
        }
        pub fn zero(rtype: TypeV, arg_types: Vec<TypeV>) -> NodeRef {
            Box::new(Lte {
                rtype: rtype,
                arg_types: arg_types,
                rhs: Null::zero(rtype),
                lhs: Null::zero(rtype),
            })
        }
    }

    impl Node for Lte {
        fn to_str(&self, indent: usize) -> String {
            " ".repeat(indent)
                + "<=\n"
                + &self.lhs.to_str(indent + 1)
                + "\n"
                + &self.rhs.to_str(indent + 1)
        }
        fn eval(&self, args: &[Type]) -> Type {
            let r = self.rhs.eval(args);
            let l = self.lhs.eval(args);
            match (r, l) {
                (Type::Float(ri), Type::Float(li)) => Type::Bool(ri <= li),
                (Type::Float(ri), Type::Int(li)) => Type::Bool(ri <= li as f64),
                (Type::Float(ri), Type::UInt(li)) => Type::Bool(ri <= li as f64),
                (Type::Int(ri), Type::Int(li)) => Type::Bool(ri <= li),
                (Type::Int(ri), Type::UInt(li)) => Type::Bool(ri <= li as i64),
                (Type::Int(ri), Type::Float(li)) => Type::Bool((ri as f64) <= li),
                (Type::UInt(ri), Type::UInt(li)) => Type::Bool(ri <= li),
                (Type::UInt(ri), Type::Int(li)) => Type::Bool((ri as i64) <= li),
                (Type::UInt(ri), Type::Float(li)) => Type::Bool((ri as f64) <= li),
                _ => panic!("Invalid: Can't Compare {:?} with {:?}", r, l),
            }
        }
        fn get_rtype(&self) -> TypeV {
            self.rtype
        }
        fn get_arg_types(&self) -> &[TypeV] {
            return &self.arg_types;
        }
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }
}

pub mod misc {
    use super::*;
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
        fn get_type_zero(&self) -> NodeRef {
            Self::zero(self.rtype, self.arg_types.clone())
        }
    }
}

pub mod math {
    use super::*;
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
                    " ".repeat(indent)
                        + stringify!($type_name)
                        + "\n"
                        + &self.arg.to_str(indent + 1)
                }

                fn get_arg_types(&self) -> &[TypeV] {
                    return &self.arg_types;
                }
                fn get_type_zero(&self) -> NodeRef {
                    Self::zero()
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
}

pub struct Tree {
    root: NodeRef,
    arg_type: Vec<TypeV>,
    rtype: TypeV,
}

pub struct BuilderTable {
    rtype_bool: Vec<NodeRef>,
    rtype_int: Vec<NodeRef>,
    rtype_uint: Vec<NodeRef>,
    rtype_float: Vec<NodeRef>,
}

impl BuilderTable {
    pub fn new() -> BuilderTable {
        BuilderTable {
            rtype_bool: vec![],
            rtype_int: vec![],
            rtype_uint: vec![],
            rtype_float: vec![],
        }
    }

    pub fn push(&mut self, ty: TypeV, node: NodeRef) {
        match ty {
            TypeV::Bool => self.rtype_bool.push(node),
            TypeV::Int => self.rtype_int.push(node),
            TypeV::UInt => self.rtype_uint.push(node),
            TypeV::Float => self.rtype_float.push(node),
        }
    }
}

pub mod btables {
    use super::*;
    pub struct FloatFnTable {
        pub table: BuilderTable,
    }

    impl FloatFnTable {
        pub fn new() -> Self {
            let mut b = Self {
                table: BuilderTable::new(),
            };
            b.table.push(TypeV::Float, math::Sin::zero());
            b.table.push(TypeV::Float, math::Cos::zero());
            b.table.push(TypeV::Float, math::Tan::zero());
            b.table.push(TypeV::Float, math::Exp::zero());
            b.table.push(TypeV::Float, math::Log::zero());
            b.table.push(TypeV::Float, math::Abs::zero());
            b.table.push(
                TypeV::Float,
                ops::Add::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
            );
            b.table.push(
                TypeV::Float,
                ops::Sub::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
            );
            b.table.push(
                TypeV::Float,
                ops::Mul::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
            );
            b.table.push(
                TypeV::Float,
                ops::Div::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
            );
            b.table.push(
                TypeV::Float,
                ops::Pow::zero(TypeV::Float, vec![TypeV::Float, TypeV::Float]),
            );

            return b;
        }
    }
}
