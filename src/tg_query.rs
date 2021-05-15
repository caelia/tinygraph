pub mod tg_data;
use tg_data::*;

// pub trait TGQuery 

pub enum QueryOp (CREATE, RETRIEVE, UPDATE, DELETE);

pub trait TGQuery {
    fn operation(&self) -> QueryOp;
    fn target(&self) -> ResourceSpec;
    fn content(&self) -> Option<TGData>;
}

pub struct CompoundQuery {
    operation: QueryOp,
    target: ResourceSpec,
    content: Option<TGData>, 
}

pub impl CompoundQuery {
    pub fn new(operation: QueryOp, target: RetrievalSpec, content: Option<TGData>) -> Self {
        CompoundQuery { operation, target, content }
    }
}

pub impl TGQuery for CompoundQuery {
    fn operation(&self) -> QueryOp {
        self.operation
    }
    fn target(&self) -> ResourceSpec {
        self.target
    }
    fn content(&self) -> Option<TGData> {
        self.content
    }
}

pub struct BaseResourceQuery {
    target: ResourceSpec,
}

pub impl TGQuery for BaseResourceQuery {
    fn operation(&self) -> QueryOp {
        RETRIEVE
    }
    fn target(&self) -> ResourceSpec {
        self.target
    }
    fn content(&self) -> Option<TGData> {
        None
    }
}

