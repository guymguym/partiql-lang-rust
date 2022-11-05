use partiql_value::{BindingsName, Value};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct OpId(usize);

impl OpId {
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct LogicalPlan<T> {
    nodes: Vec<T>,
    edges: Vec<(OpId, OpId)>,
}

impl<T> LogicalPlan<T> {
    pub fn new() -> Self {
        LogicalPlan {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_operator(&mut self, op: T) -> OpId {
        self.nodes.push(op);
        OpId(self.operator_count())
    }

    pub fn add_flow(&mut self, src: OpId, dst: OpId) {
        let src_idx = src.index() - 1;
        let dst_idx = dst.index() - 1;
        assert!(src_idx <= self.operator_count());
        assert!(dst_idx <= self.operator_count());

        self.edges.push((src, dst));
    }

    pub fn operator_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn operators(&self) -> &Vec<T> {
        &self.nodes
    }

    pub fn flows(&self) -> &Vec<(OpId, OpId)> {
        &self.edges
    }
}

// TODO: other expressions modeled in logical plan and evaluator -- IN, IS, BETWEEN

// TODO we should replace this enum with some identifier that can be looked up in a symtab/funcregistry?
#[derive(Clone, Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
}

// TODO we should replace this enum with some identifier that can be looked up in a symtab/funcregistry?
#[derive(Clone, Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BinaryOp {
    And,
    Or,
    Concat,
    Eq,
    Neq,
    Gt,
    Gteq,
    Lt,
    Lteq,

    // Arithmetic ops
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Clone, Debug)]
pub enum PathComponent {
    Key(String),
    Index(i64),
}

#[derive(Clone, Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum ValueExpr {
    // TODO other variants
    UnExpr(UnaryOp, Box<ValueExpr>),
    BinaryExpr(BinaryOp, Box<ValueExpr>, Box<ValueExpr>),
    Lit(Box<Value>),
    Path(Box<ValueExpr>, Vec<PathComponent>),
    VarRef(BindingsName),
}

// Bindings -> Bindings : Where, OrderBy, Offset, Limit, Join, SetOp, Select, Distinct, GroupBy, Unpivot, Let
// Values   -> Bindings : From
// Bindings -> Values   : Select Value

#[derive(Debug, Default)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BindingsExpr {
    From(From),
    Scan(Scan),
    Unpivot,
    Where(Where),
    OrderBy,
    Offset,
    Limit,
    Join,
    SetOp,
    SelectValue(SelectValue),
    Select(Select),
    Project(Project),
    Distinct(Distinct),
    GroupBy,
    #[default]
    Output,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BindingsToValueExpr {}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum ValueToBindingsExpr {}

/// [`From`] bridges from [`ValueExpr`]s to [`BindingExpr`]s
#[derive(Debug)]
pub struct From {
    pub expr: ValueExpr,
    pub as_key: String,
    pub at_key: Option<String>,
    pub out: Box<BindingsExpr>,
}

#[derive(Debug)]
pub struct Scan {
    pub expr: ValueExpr,
    pub as_key: String,
    pub at_key: Option<String>,
}

#[derive(Debug)]
pub struct Where {
    pub expr: ValueExpr,
    pub out: Box<BindingsExpr>,
}

#[derive(Debug)]
pub struct Select {
    pub exprs: HashMap<String, ValueExpr>,
    pub out: Box<BindingsExpr>,
}

#[derive(Debug)]
pub struct Project {
    pub exprs: HashMap<String, ValueExpr>,
}

#[derive(Debug)]
pub struct SelectValue {
    pub exprs: ValueExpr,
    pub out: Box<ValueExpr>,
}

#[derive(Debug)]
pub struct Distinct {
    pub out: Box<BindingsExpr>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        let mut p: LogicalPlan<BindingsExpr> = LogicalPlan::new();
        let a = p.add_operator(BindingsExpr::OrderBy);
        let b = p.add_operator(BindingsExpr::Output);
        let c = p.add_operator(BindingsExpr::Limit);
        p.add_flow(a, b);
        p.add_flow(a, c);
        p.add_flow(b, c);
        assert_eq!(3, p.operators().len());
        assert_eq!(3, p.flows().len());
    }
}