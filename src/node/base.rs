#![allow(dead_code)]
use rand;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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
    Int(i32),
    Float(f32),
    UInt(u32),
    Bool(bool),
}

impl Type {
    //These methods give "Zero" values of the type
    //I intend to use this as a marker type
    //I know its little wasteful, but its fine; I think
    pub fn int(val: i32) -> Type {
        Type::Int(val)
    }
    pub fn float(val: f32) -> Type {
        Type::Float(val)
    }
    pub fn uint(val: u32) -> Type {
        Type::UInt(val)
    }
    pub fn bool(val: bool) -> Type {
        Type::Bool(val)
    }

    #[allow(dead_code)]
    pub fn rand(&self) -> Type {
        match self {
            Type::Int(_) => Type::Int(rand::random()),
            Type::Float(_) => Type::Float(rand::random()),
            Type::UInt(_) => Type::UInt(rand::random()),
            Type::Bool(_) => Type::Bool(rand::random()),
        }
    }

    pub fn int_range(a: i32, b: i32) -> Type {
        let mut rng = thread_rng();
        Type::Int(rng.gen_range(a..=b))
    }
    pub fn float_range(a: f32, b: f32) -> Type {
        let mut rng = thread_rng();
        Type::Float(rng.gen_range(a..=b))
    }

    pub fn uint_range(a: u32, b: u32) -> Type {
        let mut rng = thread_rng();
        Type::UInt(rng.gen_range(a..=b))
    }
    pub fn bool_rand() -> Type {
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

#[derive(Debug)]
pub struct TypeErr {
    pub msg: String,
}

pub type NodeRef = Box<dyn Node>;

/// This is the top level Node trait
/// `Node` is not to be initiliazed directly, but
/// rather `NodeRef` that is dyn object of trait is to be used
pub trait Node {
    /// each node is evaluated and value is passed up the tree
    fn eval(&self, args: &[Type]) -> Type;
    /// returns string representation of the node in the form of tree
    fn get_tree_str(&self, indent: usize) -> String;
    /// get the string representation of expression in the form of mathematical expression
    fn get_equation_str(&self)->String;
    /// returns the return type of the node
    fn get_rtype(&self) -> TypeV;
    /// return the argument types
    fn get_arg_types(&self) -> &[TypeV];
    /// sets the nth child of the node;
    /// the exact child to be set depends on node type
    fn set_child(&mut self, child_index: usize, child: NodeRef);

    /// return a reference to nth child of the node
    fn get_child(&self, child_index: usize) -> &NodeRef;
    /// returns a Zero value for the node
    /// the zero value of the node is the one where all
    /// the children of the node are set to Null node.
    /// This is useful for building expression tree,
    /// where zero value will be intermediate form
    fn get_zero_node(&self) -> NodeRef;
    /// this method builds a random tree with
    /// the implemnted Node type as root, by calling
    /// the method recursively for children.
    /// The nodes are chosen randomly from the build_table;
    /// build_params are misc parameters required
    fn build_random_node<'a>(
        &self,
        build_table: &'a BuilderTable,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        depth: usize,
        params: &'a mut BuilderParams,
    ) -> NodeRef;
    /// recursively copies a node and its children and returns
    /// the copy
    fn deep_copy(&self) -> NodeRef;
    /// recursively copies a node and its children with a 
    /// random chance that it'll return a random node_tree
    /// instead. The chance of actual node being replaced with random
    /// branch increases as the depth of recursion increases
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        build_table: &'a BuilderTable,
        params: &'a mut BuilderParams,
    ) -> Option<NodeRef>;
    /// checks if constructed expression tree is valid
    fn type_check(&self) -> Result<(), TypeErr>;
    //these two methods are required for "conjugation" of two trees
    //to form a brand new child tree

    /// return a random child node from the tree
    /// probability increases as you go down recursively
    /// this can be null, as you might not get a child at all
    fn get_random_child(
        &self,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef>;
    /// Takes a node and randomly selects a child node and replaces it with
    /// new_node.
    /// probability of child being selected
    /// increases as you go down recursively.
    /// In case no node was selected, it will return null
    fn set_random_child(
        &self,
        new_node: NodeRef,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef>;

    /// These is to remove wasteful nodes from the tree
    /// By wasteful I mean stuff like Abs(Abs(...)) === Abs(...)
    /// or Sin(Asin(x)) == x and so on
    fn prune(&self) -> NodeRef;

    fn get_name(&self) -> &'static str;

    /// recursively calculates the depth of the deepest branch of a node
    fn get_max_depth(&self) -> usize;
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
    fn get_tree_str(&self, indent: usize) -> String {
        format!("{}{:#?}", " ".repeat(indent), self.rtype)
    }
    fn get_equation_str(&self)->String {
        unreachable!()
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn eval(&self, _args: &[Type]) -> Type {
        panic!("Cannot evaluate a Null block!!");
    }

    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn get_zero_node(&self) -> NodeRef {
        Null::zero(self.rtype)
    }
    fn set_child(&mut self, _child_index: usize, _child: NodeRef) {
        panic!("Cannot set child node for Null node!!");
    }
    fn build_random_node<'a>(
        &self,
        _build_table: &'a BuilderTable,
        _arg_types: &[TypeV],
        node_rtype: TypeV,
        _depth: usize,
        _params: &'a mut BuilderParams,
    ) -> NodeRef {
        Null::zero(node_rtype)
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        Err(TypeErr {
            msg: "Null node is invalid!!".to_string(),
        })
    }
    fn deep_copy(&self) -> NodeRef {
        Null::zero(self.rtype)
    }
    fn mutant_copy<'a>(
        &self,
        _probability: f32,
        _node_depth: usize,
        _arg_types: &[TypeV],
        _build_table: &'a BuilderTable,
        _params: &'a mut BuilderParams,
    ) -> Option<NodeRef> {
        None
    }

    fn get_random_child(
        &self,
        _probability: f32,
        _depth: usize,
        _params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        None
    }

    fn set_random_child(
        &self,
        _new_node: NodeRef,
        _probability: f32,
        _depth: usize,
        _params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        None
    }

    fn prune(&self) -> NodeRef {
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "NULL"
    }

    fn get_child(&self, _child_index: usize) -> &NodeRef {
        unreachable!()
    }

    fn get_max_depth(&self) -> usize {
        0
    }
}

/// Val node for storing constant values
pub struct Val {
    rtype: TypeV,
    pub v: Type,
    arg_types: Vec<TypeV>,
}

impl Val {
    pub fn make(val: Type) -> NodeRef {
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
    pub fn zero(rtype: TypeV) -> NodeRef {
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

impl Node for Val {
    fn get_tree_str(&self, indent: usize) -> String {
        format!("{}{}", " ".repeat(indent), self.v)
    }
    ///returns equation in string format
    fn get_equation_str(&self)->String {
        format!("{}", self.v)
    }
    /// On evaluation, value returns constant it represents
    fn eval(&self, _: &[Type]) -> Type {
        self.v
    }

    fn get_rtype(&self) -> TypeV {
        self.rtype
    }

    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, _child_index: usize, _child: NodeRef) {
        panic!("Cannot set child node for Val node!!");
    }
    fn get_child(&self, _child_index: usize) -> &NodeRef {
        unreachable!()
    }
    fn get_zero_node(&self) -> NodeRef {
        Self::zero(self.rtype)
    }
    fn build_random_node<'a>(
        &self,
        _build_table: &'a BuilderTable,
        _arg_types: &[TypeV],
        node_rtype: TypeV,
        _depth: usize,
        _params: &'a mut BuilderParams,
    ) -> NodeRef {
        let val = Type::random(node_rtype);
        Val::make(val)
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        Ok(())
    }
    fn deep_copy(&self) -> NodeRef {
        Self::make(self.v)
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        _arg_types: &[TypeV],
        _build_table: &'a BuilderTable,
        params: &'a mut BuilderParams,
    ) -> Option<NodeRef> {
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

    fn get_random_child(
        &self,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth) {
            Some(self.deep_copy())
        } else {
            None
        }
    }

    fn set_random_child(
        &self,
        new_node: NodeRef,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth) {
            Some(new_node)
        } else {
            None
        }
    } 

    fn prune(&self) -> NodeRef {
        self.deep_copy()
    }

    fn get_name(&self) -> &'static str {
        "Val"
    }

    fn get_max_depth(&self) -> usize {
        1
    }
}

pub struct Var {
    rtype: TypeV,
    pub idx: usize,
    arg_types: Vec<TypeV>,
}

impl Var {
    pub fn make(idx: usize, rtype: TypeV) -> NodeRef {
        Box::new(Var {
            idx,
            rtype,
            arg_types: vec![],
        })
    }
}

impl Node for Var {
    fn get_tree_str(&self, indent: usize) -> String {
        format!("{}x[{}]", " ".repeat(indent), self.idx)
    }
    ///returns equation in string format
    fn get_equation_str(&self)->String {
        format!("x[{}]", self.idx)
    }
    fn eval(&self, args: &[Type]) -> Type {
        args[self.idx]
    }
    fn get_rtype(&self) -> TypeV {
        self.rtype
    }
    fn get_arg_types(&self) -> &[TypeV] {
        &self.arg_types
    }
    fn set_child(&mut self, _child_index: usize, _child: NodeRef) {
        panic!("Cannot set child node for Var node!!");
    }
    fn get_child(&self, _child_index: usize) -> &NodeRef {
        unreachable!()
    }
    fn get_zero_node(&self) -> NodeRef {
        Self::make(0, self.rtype)
    }
    fn build_random_node<'a>(
        &self,
        _build_table: &'a BuilderTable,
        arg_types: &[TypeV],
        node_rtype: TypeV,
        _depth: usize,
        params: &'a mut BuilderParams,
    ) -> NodeRef {
        let valid_indices: Vec<_> = (0..arg_types.len())
            .filter(|x| arg_types[*x] == node_rtype) //Only arguments with same type as rtype are to be chosen
            .collect();
        let vindex = *valid_indices.choose(&mut params.randomizer).unwrap();
        Var::make(vindex, node_rtype)
    }
    fn type_check(&self) -> Result<(), TypeErr> {
        Ok(())
    }
    fn deep_copy(&self) -> NodeRef {
        Self::make(self.idx, self.rtype)
    }
    fn mutant_copy<'a>(
        &self,
        probability: f32,
        node_depth: usize,
        arg_types: &[TypeV],
        _build_table: &'a BuilderTable,
        params: &'a mut BuilderParams,
    ) -> Option<NodeRef> {
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
    fn get_random_child(
        &self,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth) {
            Some(self.deep_copy())
        } else {
            None
        }
    }

    fn set_random_child(
        &self,
        new_node: NodeRef,
        probability: f32,
        depth: usize,
        params: &'_ mut BuilderParams,
    ) -> Option<NodeRef> {
        if params.randomizer.gen::<f32>() < params.get_mut_prob(probability, depth) {
            Some(new_node)
        } else {
            None
        }
    }

    fn prune(&self) -> NodeRef {
        self.deep_copy()
    }
    fn get_name(&self) -> &'static str {
        "Var"
    }

    fn get_max_depth(&self) -> usize {
        1
    }
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
    pub max_depth: usize,
    pub randomizer: rand::prelude::ThreadRng,
    pub termination_probability: f32,
    pub float_range: (f32, f32),
    pub int_range: (i32, i32),
    pub uint_range: (u32, u32),
}

impl BuilderTable {
    pub fn new() -> BuilderTable {
        BuilderTable {
            rtype_bool: vec![],
            rtype_int: vec![],
            rtype_uint: vec![],
            rtype_float: vec![],
            val_node: Val::zero(TypeV::Bool),
            var_node: Var::make(0, TypeV::Bool),
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

    pub fn get_rand_node(
        &self,
        depth: usize,
        rtype: TypeV,
        params: &mut BuilderParams,
    ) -> &NodeRef {
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

/// Builder pattern for BuilderParams
impl BuilderParams {
    pub fn new() -> BuilderParams {
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

    pub fn set_max_depth(&mut self, val: usize) {
        self.max_depth = val;
    }
    pub fn set_float_range(&mut self, a: f32, b: f32) {
        self.float_range = (a, b);
    }
    pub fn set_int_range(&mut self, a: i32, b: i32) {
        self.int_range = (a, b);
    }

    pub fn set_uint_range(&mut self, a: u32, b: u32) {
        self.uint_range = (a, b);
    }
    pub fn set_termination_probability(&mut self, val: f32) {
        self.termination_probability = val;
    }
    pub fn set_seed(&mut self, seed: u64) {
        let mut array = [seed];
        self.randomizer.fill(&mut array);
    }
    pub fn get_mut_prob(&self, base_prob: f32, depth: usize) -> f32 {
        let s = (usize::pow(2, depth as u32) as f32) * base_prob;
        if depth >= self.max_depth {
            0.0
        } else {
            1.0 / (1.0 + f32::exp(-5.5 * (s - 0.7)))
        } //FIXME: Maybe this is not a good function for probability growth
    }
}
