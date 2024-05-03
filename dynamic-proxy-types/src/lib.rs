use std::any::{Any, TypeId};
use std::ops::Deref;

/// Contains invocation arguments, including function signature and arguments
pub struct InvocationInfo<'a> {
    /// Gets the name of the invoked function
    pub func_name: &'a str,
    /// Array containig the name of input arguments
    pub arg_names: &'a[&'a str],
    /// Array containing the values of the arguments as boxed dynamic value.
    pub arg_values: &'a [Box<dyn Any>],
    /// Type id of the function result
    pub return_type: TypeId,
    /// The interceptor must set this value if the function has result
    pub return_value: Option<Box<dyn Any>>
}

pub trait DynamicProxy {
    fn call(&self, invocation: &mut InvocationInfo);
}

pub trait AsyncDynamicProxy {
    async fn call_async(&self, invocation: &mut InvocationInfo);
}


impl<'a> InvocationInfo<'a> {
    
    pub fn set_return_value<T: 'static>(&mut self, val: T) {
        let result: Box<dyn Any> = Box::new(val);
        self.return_value = Some(result);
    }
    
    pub fn get_arg_value<T: 'static>(&self, index: usize) -> &T {
        self.arg_values[index].downcast_ref::<T>().unwrap()
    }
    
    pub fn get_arg_type(&self, index: usize) -> TypeId {
        self.arg_values[index].deref().type_id()
    }
}