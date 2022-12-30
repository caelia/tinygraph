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
        node_id INTEGER,
        relation REFERENCES relations(id) NOT NULL,
        type REFERENCE types(id) NOT NULL,
        value TEXT NOT NULL
    );
";

pub const POPULATE_TYPE_TABLE: [&str; 8] = [
    "INSERT INTO types (typename) VALUES ('int');",
    "INSERT INTO types (typename) VALUES ('float');",
    "INSERT INTO types (typename) VALUES ('bool');",
    "INSERT INTO types (typename) VALUES ('string');",
    "INSERT INTO types (typename) VALUES ('datetime');",
    "INSERT INTO types (typename) VALUES ('iref');",
    "INSERT INTO types (typename) VALUES ('lref');",
    "INSERT INTO types (typename) VALUES ('rref');"
];
