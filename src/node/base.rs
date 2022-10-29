use rand;
use rand::seq::SliceRandom;
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
    fn set_child(&mut self, child_index: usize, child: NodeRef);
    fn get_type_zero(&self) -> NodeRef;
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams,
    ) -> NodeRef;
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
    fn set_child(&mut self, child_index: usize, child: NodeRef) {
        panic!("Cannot set child node for Null node!!");
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams,
    ) -> NodeRef {
        Null::zero(node_rtype)
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
    fn set_child(&mut self, child_index: usize, child: NodeRef) {
        panic!("Cannot set child node for Val node!!");
    }
    fn get_type_zero(&self) -> NodeRef {
        Self::zero(self.rtype)
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams,
    ) -> NodeRef {
        let val = Randomize::random(node_rtype);
        Val::new(val)
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
    fn set_child(&mut self, child_index: usize, child: NodeRef) {
        panic!("Cannot set child node for Var node!!");
    }
    fn get_type_zero(&self) -> NodeRef {
        Self::new(0, self.rtype)
    }
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams,
    ) -> NodeRef {
        let valid_indices: Vec<_> = (0..arg_types.len())
            .filter(|x| arg_types[*x] == node_rtype) //Only arguments with same type as rtype are to be chosen
            .collect();
        let vindex = *valid_indices.choose(&mut rand::thread_rng()).unwrap();
        return Var::new(vindex, node_rtype);
    }
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
    val_node: NodeRef,
    var_node: NodeRef,
}

pub struct BuilderParams {
    max_depth: usize,
    randomizer: rand::prelude::ThreadRng,
    termination_probability: f32,
    float_range: (f32, f32),
    int_range: (i32, i32),
    uint_range: (u32, u32),
}

impl BuilderTable {
    pub fn new() -> BuilderTable {
        BuilderTable {
            rtype_bool: vec![],
            rtype_int: vec![],
            rtype_uint: vec![],
            rtype_float: vec![],
            val_node: Val::zero(TypeV::Bool),
            var_node: Var::new(0, TypeV::Bool),
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

    pub fn get_rand_node<'a>(
        &self,
        depth: usize,
        rtype: TypeV,
        params: &'a mut BuilderParams,
    ) -> &NodeRef {
        if (params.randomizer.gen::<f32>() <= params.termination_probability)
            || (depth >= params.max_depth)
        {
            if params.randomizer.gen::<f32>() <= 0.5 {
                &self.val_node
            } else {
                &self.var_node
            }
        } else {
            return match rtype {
                TypeV::Bool => self.rtype_bool.choose(&mut params.randomizer).unwrap(),
                TypeV::Int => self.rtype_int.choose(&mut params.randomizer).unwrap(),
                TypeV::UInt => self.rtype_uint.choose(&mut params.randomizer).unwrap(),
                TypeV::Float => self.rtype_float.choose(&mut params.randomizer).unwrap(),
            };
        }
    }
}

/// Builder pattern for BuilderParams
impl BuilderParams {
    pub fn new() -> BuilderParams {
        BuilderParams {
            max_depth: 10,                 //Set this value as default
            termination_probability: 0.05, //set early termination probabilty as 5% in the beginning
            float_range: (0.0, 1.0),
            int_range: (-100, 100),
            uint_range: (0, 100),
            randomizer: thread_rng(),
        }
    }

    pub fn max_depth(mut self, val: usize) -> Self {
        self.max_depth = val;
        self
    }

    pub fn termination_probability(mut self, val: f32) -> Self {
        self.termination_probability = val;
        self
    }

    pub fn float_range(mut self, a: f32, b: f32) -> Self {
        self.float_range = (a, b);
        self
    }

    pub fn int_range(mut self, a: i32, b: i32) -> Self {
        self.int_range = (a, b);
        self
    }

    pub fn uint_range(mut self, a: u32, b: u32) -> Self {
        self.uint_range = (a, b);
        self
    }
}
