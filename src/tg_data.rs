pub enum DataType (
    INTEGER,
    FLOAT,
    BOOLEAN,
    STRING,
    DATE,
    TIME,
    PERIOD,
    )

// pub enum ResourceType (NODE, EDGE, LITERAL);

// I *think* the purpose of this type is to identify resources within a single
// database. So let's simplify it. References to remote resources will be handled
// separately.
/*
pub enum ResourceSpec (
    INTERNAL_NODE(u32),
    INTERNAL_EDGE(u32),
    LOCAL_NODE(String, u32),
    LOCAL_EDGE(String, u32),
    REMOTE_NODE(String),
    REMOTE_EDGE(String),
    LITERAL(DataType, String),
    )
*/

pub enum ResourceSpec (
    NODE(u32),
    EDGE(u32),
    LITERAL(DataType, String),
)

/*
struct ResourceSpec {
    restype: ResourceType,
    id: u32,
}
*/

struct TGData {};
