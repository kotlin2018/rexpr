use std::sync::{RwLock, TryLockResult, RwLockReadGuard, RwLockWriteGuard};
use crate::ast::Node;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

#[derive(Debug)]
pub struct RWLockMapCache{
    pub shared:RwLock<std::collections::HashMap<String,Node>>
}

impl RWLockMapCache{
    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, HashMap<String, Node, RandomState>>> {
        self.shared.try_read()
    }
    pub fn try_write(&self)->TryLockResult<RwLockWriteGuard<'_, HashMap<String, Node, RandomState>>>{
        self.shared.try_write()
    }
}