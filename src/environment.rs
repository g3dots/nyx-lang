use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => self.outer.as_ref()?.borrow().get(name),
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name, val.clone());
        val
    }

    pub fn update(&mut self, name: &str, val: Object) -> Option<Object> {
        if self.store.contains_key(name) {
            self.store.insert(name.to_string(), val.clone());
            return Some(val);
        }
        if let Some(outer) = &self.outer {
            return outer.borrow_mut().update(name, val);
        }
        None
    }
}
