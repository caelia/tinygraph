pub const CREATE_REL_TABLE: &str = "
    CREATE TABLE relations (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        relname TEXT NOT NULL,
        bidi INTEGER,
        inverse TEXT
    );
";

pub const CREATE_TYPE_TABLE: &str = "
    CREATE TABLE types (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        typename TEXT NOT NULL
    );
";

pub const CREATE_DATA_TABLE: &str = "
    CREATE TABLE data (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        node_id INTEGER NOT NULL,
        relation REFERENCES relations(id) NOT NULL,
        type REFERENCES types(id) NOT NULL,
        value TEXT NOT NULL
    );
";

pub const POPULATE_TYPE_TABLE: &str = "INSERT INTO types (typename) VALUES (?);";
