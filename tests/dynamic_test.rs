#[cfg(test)]
mod tests {
    use std::any::Any;
    use dynamic_proxy::dynamic_proxy;
    use dynamic_proxy_types::{DynamicProxy, InvocationInfo};
    use std::ops::Deref;
    // use crate::MyTrait;

    pub struct Interceptor;

    impl DynamicProxy for Interceptor {
        fn call(self: &Self, invocation: &mut InvocationInfo){
            let result: Box<dyn Any> = Box::new(invocation.func_name.to_string());
            invocation.return_value = Some(result);
        }
    }
    
    #[dynamic_proxy(Interceptor)]
    pub trait MyTrait {
        fn add(self, a: i32, b: i32) -> String;
        fn subtract(self, a: i32, b: i32) -> String;
    }

    #[test]
    fn function_name() {
        use crate::tests::Interceptor;
        // use crate::tests::MyTrait;
        let s = Interceptor {};
        assert_eq!(s.add(6, 7), "add");
    }
}
