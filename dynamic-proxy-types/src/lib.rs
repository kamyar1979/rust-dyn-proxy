use std::any::{Any, TypeId};
// use syn::{Signature, TraitItemFn};

pub struct InvocationInfo<'a> {
    pub func_name: &'a str,
    pub arg_names: Vec<&'a str>,
    pub arg_values: Vec<Box<dyn Any>>,
    pub return_value: Option<Box<dyn Any>>
}

pub trait DynamicProxy {
    fn call(self: &Self, invocation: &mut InvocationInfo);
}
