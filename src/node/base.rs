#![allow(dead_code)]
use std::marker::PhantomData;
use rand;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use num::{Integer, Unsigned, Float};


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
pub enum Type<F:Float, I:Integer, U:Unsigned> {
    Float(F),
    Int(I),
    UInt(U),
    Bool(bool),
}

impl<F:Float, I:Integer, U:Unsigned> Type<F,I,U> {
    //These methods give "Zero" values of the type
    //I intend to use this as a marker type
    //I know its little wasteful, but its fine; I think
    pub fn int(val: I) -> Self {
        Type::Int(val)
    }
    pub fn float(val: F) -> Self {
        Type::Float(val)
    }
    pub fn uint(val: U) -> Self{
        Type::UInt(val)
    }
    pub fn bool(val: bool) -> Self {
        Type::Bool(val)
    }

    #[allow(dead_code)]
    pub fn rand(&self) -> Self {
        match self {
            Type::Int(_) => Type::Int(rand::random()),
            Type::Float(_) => Type::Float(rand::random()),
            Type::UInt(_) => Type::UInt(rand::random()),
            Type::Bool(_) => Type::Bool(rand::random()),
        }
    }

    pub fn int_range(a: I, b: I) -> Self {
        let mut rng = thread_rng();
        Type::Int(rng.gen_range(a..=b))
    }
    pub fn float_range(a: F, b: F) -> Self {
        let mut rng = thread_rng();
        Type::Float(rng.gen_range(a..=b))
    }

    pub fn uint_range(a: U, b: U) -> Self {
        let mut rng = thread_rng();
        Type::UInt(rng.gen_range(a..=b))
    }
    pub fn bool_rand() -> Self {
        Type::Bool(rand::random())
    }
    fn random(rtype: TypeV) -> Self {
        match rtype {
            TypeV::Int => Type::Int(rand::random()),
            TypeV::Float => Type::Float(rand::random()),
            TypeV::UInt => Type::UInt(rand::random()),
            TypeV::Bool => Type::Bool(rand::random()),
        }
    }
}

impl<F:Float, I:Integer, U:Unsigned> std::fmt::Display for Type<F,I,U> {
    fn fmt(&self, ft: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int(i) => write!(ft, "{}I", i),
            Type::Float(f) => write!(ft, "{}F", f),
            Type::UInt(u) => write!(ft, "{}U", u),
            Type::Bool(b) => write!(ft, "{}", b),
        }
    }
}

#[derive(Debug)]
pub struct TypeErr {
    pub msg: String,
}

pub type NodeRef<F:Float, I:Integer, U:Unsigned> = Box<dyn Node<F,I,U>>;

/// This is the top level Node trait
/// `Node` is not to be initiliazed directly, but
/// rather `NodeRef` that is dyn object of trait is to be used
pub trait Node<F:Float, I:Integer, U:Unsigned> {
    /// each node is evaluated and value is passed up the tree
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U>;
    fn to_str(&self, indent: usize) -> String;
    fn get_rtype(&self) -> TypeV;
    fn get_arg_types(&self) -> &[TypeV];
    fn set_child(&mut self, child_index: usize, child: NodeRef<F,I,U>);
    fn get_type_zero(&self) -> NodeRef<F,I,U>;
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable<F,I,U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U>;
    fn deep_copy(&self) -> NodeRef<F,I,U>;
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>>;
    fn type_check(&self) -> Result<(), TypeErr>;
}

/// Special FnNode trait for function node
/// They will define fn_eval function, which'll be used
/// instead of having function member or other mechanisms
pub trait FnNode<F:Float, I:Integer, U:Unsigned>: Node<F,I,U> {
    //fn get_arg_types(&self) -> Vec<TypeV>;
    fn set_args(&mut self, args: Vec<NodeRef<F,I,U>>);
}

/// A NUll node. This node does nothing
/// used only for setting "zero" node
pub struct Null<F:Float, I:Integer, U:Unsigned> {
    rtype: TypeV,
    arg_types: Vec<TypeV>,
    val: Type<F,I,U>
}

impl<F:Float, I:Integer, U:Unsigned> Null<F,I,U> {
    pub fn zero(rtype: TypeV) -> NodeRef<F,I,U> {
        Box::new(Null {
            rtype,
            arg_types: vec![],
            val: Type::Bool(false)
        })
    }
}

impl<F:Float, I:Integer, U:Unsigned> Node<F,I,U> for Null<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        format!("{}{:#?}", " ".repeat(indent), self.rtype)
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn eval(&self, _args: &[Type<F,I,U>]) -> Type<F,I,U> {
        panic!("Cannot evaluate a Null block!!");
    }

    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Null::zero(self.rtype)
    }
    fn set_child(&mut self, _child_index: usize, _child: NodeRef<F,I,U>) {
        panic!("Cannot set child node for Null node!!");
    }
    fn build_random_node<'a>(
        &self,
        _build_table: &'a BuilderTable<F,I,U>,
        _arg_types: &[TypeV],
        node_rtype: TypeV,
        _depth: usize,
        _params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
        Null::zero(node_rtype)
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        Err(TypeErr {
            msg: "Null node is invalid!!".to_string(),
        })
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Null::zero(self.rtype)
    }
    fn mutant_copy<'a>(
        &self,
        _probability: f32,
        _node_depth: usize,
        _arg_types: &[TypeV],
        _build_table: &'a BuilderTable<F,I,U>,
        _params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
        None
    }
}

/// Val node for storing constant values
pub struct Val<F:Float, I:Integer, U:Unsigned> {
    rtype: TypeV,
    pub v: Type<F,I,U>,
    arg_types: Vec<TypeV>,
}

impl<F:Float, I:Integer, U:Unsigned> Val<F,I,U> {
    pub fn make(val: Type<F,I,U>) -> NodeRef<F,I,U> {
        let rtype = match val {
            Type::Bool(_) => TypeV::Bool,
            Type::UInt(_) => TypeV::UInt,
            Type::Int(_) => TypeV::Int,
            Type::Float(_) => TypeV::Float,
        };
        Box::new(Val {
            v: val,
            rtype,
            arg_types: vec![],
        })
    }
    pub fn zero(rtype: TypeV) -> NodeRef<F,I,U> {
        let v = match rtype {
            TypeV::Int => Type::int(0),
            TypeV::Float => Type::float(0.0),
            TypeV::UInt => Type::uint(0),
            TypeV::Bool => Type::bool(false),
        };
        Box::new(Val {
            v,
            rtype,
            arg_types: vec![],
        })
    }
}

impl<F:Float, I:Integer, U:Unsigned> Node<F,I,U> for Val<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        format!("{}{}", " ".repeat(indent), self.v)
    }
    /// On evaluation, value returns constant it represents
    fn eval(&self, _: &[Type<F,I,U>]) -> Type<F,I,U> {
        self.v
    }

    fn get_rtype(&self) -> TypeV {
        self.rtype
    }

    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, _child_index: usize, _child: NodeRef<F,I,U>) {
        panic!("Cannot set child node for Val node!!");
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::zero(self.rtype)
    }
    fn build_random_node<'a>(
        &self,
        _build_table: &'a BuilderTable<F,I,U>,
        _arg_types: &[TypeV],
        node_rtype: TypeV,
        _depth: usize,
        _params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
        let val = Type::random(node_rtype);
        Val::make(val)
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        Ok(())
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Self::make(self.v)
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        _arg_types: &[TypeV],
        _build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, node_depth) {
            Some(match self.v {
                Type::Float(_) => Self::make(Type::Float(
                    params
                        .randomizer
                        .gen_range(params.float_range.0..=params.float_range.1),
                )),
                _ => unimplemented!(),
            })
        } else {
            None
        }
    }
}

pub struct Var<F:Float, I:Integer, U:Unsigned> {
    rtype: TypeV,
    pub idx: usize,
    arg_types: Vec<TypeV>,
    _mark: PhantomData<(F,I,U)>
}

impl<F:Float, I:Integer, U:Unsigned> Var<F,I,U> {
    pub fn make(idx: usize, rtype: TypeV) -> NodeRef<F,I,U> {
        Box::new(Var {
            idx,
            rtype,
            arg_types: vec![],
            _mark: PhantomData::default()
        })
    }
}

impl<F:Float, I:Integer, U:Unsigned> Node<F,I,U> for Var<F,I,U> {
    fn to_str(&self, indent: usize) -> String {
        format!("{}x[{}]", " ".repeat(indent), self.idx)
    }
    fn eval(&self, args: &[Type<F,I,U>]) -> Type<F,I,U> {
        args[self.idx]
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, _child_index: usize, _child: NodeRef<F,I,U>) {
        panic!("Cannot set child node for Var node!!");
    }
    fn get_type_zero(&self) -> NodeRef<F,I,U> {
        Self::make(0, self.rtype)
    }
    fn build_random_node<'a>(
        &self,
        _build_table: &'a BuilderTable<F,I,U>,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        _depth: usize,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> NodeRef<F,I,U> {
        let valid_indices: Vec<_> = (0..arg_types.len())
            .filter(|x| arg_types[*x] == node_rtype) //Only arguments with same type as rtype are to be chosen
            .collect();
        let vindex = *valid_indices.choose(&mut params.randomizer).unwrap();
        Var::make(vindex, node_rtype)
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        Ok(())
    }
    fn deep_copy(&self) -> NodeRef<F,I,U> {
        Self::make(self.idx, self.rtype)
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        _build_table: &'a BuilderTable<F,I,U>,
        params: &'a mut BuilderParams<F,I,U>,
    ) -> Option<NodeRef<F,I,U>> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, node_depth) {
            let valid_indices: Vec<_> = (0..arg_types.len())
                .filter(|x| arg_types[*x] == self.rtype) //Only arguments with same type as rtype are to be chosen
                .collect();
            let vindex = *valid_indices.choose(&mut params.randomizer).unwrap();
            if vindex == self.idx {
                None
            } else {
                Some(Self::make(vindex, self.rtype))
            }
        } else {
            None
        }
    }
}

pub struct BuilderTable<F:Float, I:Integer, U:Unsigned> {
    rtype_bool: Vec<NodeRef<F,I,U>>,
    rtype_int: Vec<NodeRef<F,I,U>>,
    rtype_uint: Vec<NodeRef<F,I,U>>,
    rtype_float: Vec<NodeRef<F,I,U>>,
    val_node: NodeRef<F,I,U>,
    var_node: NodeRef<F,I,U>,
}

pub struct BuilderParams<F:Float, I:Integer, U:Unsigned> {
    pub max_depth: usize,
    pub randomizer: rand::prelude::ThreadRng,
    pub termination_probability: f32,
    pub float_range: (F, F),
    pub int_range: (I, I),
    pub uint_range: (U, U),
}

impl<F:Float, I:Integer, U:Unsigned> BuilderTable<F,I,U> {
    pub fn new() -> BuilderTable<F,I,U> {
        BuilderTable {
            rtype_bool: vec![],
            rtype_int: vec![],
            rtype_uint: vec![],
            rtype_float: vec![],
            val_node: Val::zero(TypeV::Bool),
            var_node: Var::make(0, TypeV::Bool),
        }
    }

    pub fn push(&mut self, ty: TypeV, node: NodeRef<F,I,U>) {
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
        params: &'a mut BuilderParams<F,I,U>,
    ) -> &NodeRef<F,I,U> {
        if (params.randomizer.gen::<f32>() <= params.termination_probability)
            || (depth >= params.max_depth)
        {
            if params.randomizer.gen::<f32>() >= 0.5 {
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

/// Builder pattern for BuilderParams<F,I,U>
impl<F:Float, I:Integer, U:Unsigned> BuilderParams<F,I,U> {
    pub fn new() -> BuilderParams<F,I,U> {
        BuilderParams {
            max_depth: 10,                 //Set this value as default
            termination_probability: 0.05, //set early termination probability as 5% in the beginning
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

    pub fn float_range(mut self, a: F, b: F) -> Self {
        self.float_range = (a, b);
        self
    }

    pub fn int_range(mut self, a: I, b: I) -> Self {
        self.int_range = (a, b);
        self
    }

    pub fn uint_range(mut self, a: U, b: U) -> Self {
        self.uint_range = (a, b);
        self
    }

    pub fn set_max_depth(&mut self, val: usize) {
        self.max_depth = val;
    }
    pub fn set_float_range(&mut self, a: F, b: F) {
        self.float_range = (a, b);
    }
    pub fn set_int_range(&mut self, a: I, b: I) {
        self.int_range = (a, b);
    }

    pub fn set_uint_range(&mut self, a: U, b: U) {
        self.uint_range = (a, b);
    }
    pub fn set_termination_probability(&mut self, val: f32) {
        self.termination_probability = val;
    }
    pub fn get_mut_prob(&self, base_prob: f32, depth: usize) -> f32 {
        let s = (usize::pow(2, depth as u32) as f32) * base_prob;
        1.0 / (1.0 + f32::exp(-5.5 * (s - 0.7))) //FIXME: Maybe this is not a good function for probability growth
    }
}
