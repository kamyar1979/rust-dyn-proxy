use std::any::Any;
use syn::TraitItemFn;

pub struct InvocationInfo {
    arguments: Vec<Box<dyn Any>>,
    method: TraitItemFn
}

pub trait DynamicProxy {
    fn call<T>(invocation: InvocationInfo);
}
