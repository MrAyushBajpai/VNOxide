use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f32),
    Bool(bool),
    Text(String),
}

#[derive(Resource, Default)]
pub struct VarStore {
    pub vars: HashMap<String, Value>,
}

impl VarStore {
    pub fn set(&mut self, key: &str, value: Value) {
        self.vars.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.vars.get(key)
    }
}
