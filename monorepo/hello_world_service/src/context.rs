use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Context(pub Arc<InnerContext>);

impl Context {
    pub async fn new() -> Self {
        Self(Arc::new(InnerContext::new().await))
    }
}


#[derive(Debug)]
pub struct InnerContext {
    pub counter: Mutex<i32>,
}

impl InnerContext {
    pub async fn new() -> Self {
        Self { 
            counter: Mutex::default(),
        }
    }
}