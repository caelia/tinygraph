pub enum ResourceType (NODE, EDGE, LITERAL);

struct ResourceSpec {
    restype: ResourceType,
    id: u32,
}

struct TGData {};
