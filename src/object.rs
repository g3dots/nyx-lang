use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::{BlockStatement, Identifier};
use crate::environment::Environment;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    StringKey(String),
}

impl fmt::Display for HashKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashKey::Integer(v) => write!(f, "{v}"),
            HashKey::Boolean(v) => write!(f, "{v}"),
            HashKey::StringKey(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    StringObj(String),
    ReturnValue(Box<Object>),
    Error(String),
    Function {
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: Rc<RefCell<Environment>>,
    },
    Builtin(String, fn(Vec<Object>) -> Object),
    Array(Vec<Object>),
    Hash(HashMap<HashKey, Object>),
    Null,
}

impl Object {
    pub fn type_name(&self) -> &str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::StringObj(_) => "STRING",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Error(_) => "ERROR",
            Object::Function { .. } => "FUNCTION",
            Object::Builtin(_, _) => "BUILTIN",
            Object::Array(_) => "ARRAY",
            Object::Hash(_) => "HASH",
            Object::Null => "NULL",
        }
    }

    pub fn as_hash_key(&self) -> Option<HashKey> {
        match self {
            Object::Integer(v) => Some(HashKey::Integer(*v)),
            Object::Boolean(v) => Some(HashKey::Boolean(*v)),
            Object::StringObj(v) => Some(HashKey::StringKey(v.clone())),
            _ => None,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::StringObj(a), Object::StringObj(b)) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Error(a), Object::Error(b)) => a == b,
            (Object::Array(a), Object::Array(b)) => a == b,
            (Object::ReturnValue(a), Object::ReturnValue(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(v) => write!(f, "{v}"),
            Object::Boolean(v) => write!(f, "{v}"),
            Object::StringObj(v) => write!(f, "{v}"),
            Object::ReturnValue(v) => write!(f, "{v}"),
            Object::Error(msg) => write!(f, "ERROR: {msg}"),
            Object::Function {
                parameters, body, ..
            } => {
                let params = parameters
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "fn({params}) {{{body}}}")
            }
            Object::Builtin(name, _) => write!(f, "builtin function: {name}"),
            Object::Array(elements) => {
                let elems = elements
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{elems}]")
            }
            Object::Hash(pairs) => {
                let entries = pairs
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{{{entries}}}")
            }
            Object::Null => write!(f, "null"),
        }
    }
}
