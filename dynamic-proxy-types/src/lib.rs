use std::any::{Any, TypeId};

pub struct InvocationInfo<'a> {
    pub func_name: &'a str,
    pub arg_names: &'a[&'a str],
    pub arg_values: &'a [Box<dyn Any>],
    pub return_type: TypeId,
    pub return_value: Option<Box<dyn Any>>
}

pub trait DynamicProxy {
    fn call(&self, invocation: &mut InvocationInfo);
}

impl<'a> InvocationInfo<'a> {
    pub fn set_return_value<T: 'static>(&mut self, val: T) {
        let result: Box<dyn Any> = Box::new(val);
        self.return_value = Some(result);
    }
}