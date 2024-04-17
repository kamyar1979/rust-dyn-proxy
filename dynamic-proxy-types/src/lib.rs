use std::any::Any;
// use syn::{Signature, TraitItemFn};

pub struct InvocationInfo<'a> {
    // arguments: Vec<Box<dyn Any>>,
    pub func_name: &'a str,
    pub return_value: Option<Box<dyn Any>>
}

pub trait DynamicProxy {
    fn call(self: &Self, invocation: &mut InvocationInfo);
}
