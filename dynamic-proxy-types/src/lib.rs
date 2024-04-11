// use std::any::Any;
use syn::{Signature, TraitItemFn};

pub struct InvocationInfo<'a> {
    // arguments: Vec<Box<dyn Any>>,
    pub func_name: &'a str
}

pub trait DynamicProxy {
    fn call<T>(self: &Self, invocation: InvocationInfo) -> usize;
}
