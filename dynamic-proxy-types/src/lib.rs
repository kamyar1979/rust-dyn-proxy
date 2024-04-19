use std::any::{Any, TypeId};

pub struct InvocationInfo<'a> {
    pub func_name: &'a str,
    pub arg_names: Vec<&'a str>,
    pub arg_values: Vec<Box<dyn Any>>,
    pub return_type: TypeId,
    pub return_value: Option<Box<dyn Any>>
}

pub trait DynamicProxy {
    fn call(&self, invocation: &mut InvocationInfo);
}
