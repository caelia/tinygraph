use crate::tg_data::*;

// pub trait TGQuery 

#[derive(Debug, Clone)]
pub enum QueryOp {
    Create,
    Retrieve,
    Update,
    Delete
}

pub trait TGQuery {
    fn operation(&self) -> &QueryOp;
    fn target(&self) -> &ResourceSpec;
    fn content(&self) -> &Option<TGData>;
}

pub struct CompoundQuery {
    operation: QueryOp,
    target: ResourceSpec,
    content: Option<TGData>, 
}

impl CompoundQuery {
    pub fn new(operation: QueryOp, target: ResourceSpec, content: Option<TGData>) -> Self {
        CompoundQuery { operation, target, content }
    }
}

impl TGQuery for CompoundQuery {
    fn operation(&self) -> &QueryOp {
        &self.operation
    }
    fn target(&self) -> &ResourceSpec {
        &self.target
    }
    fn content(&self) -> &Option<TGData> {
        &self.content
    }
}

pub struct BaseResourceQuery {
    target: ResourceSpec,
}

impl TGQuery for BaseResourceQuery {
    fn operation(&self) -> &QueryOp {
        &QueryOp::Retrieve
    }
    fn target(&self) -> &ResourceSpec {
        &self.target
    }
    fn content(&self) -> &Option<TGData> {
        &None
    }
}

