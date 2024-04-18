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
            let a = invocation.args.get(0).unwrap().downcast_ref::<i32>().unwrap();
            let b = invocation.args.get(1).unwrap().downcast_ref::<i32>().unwrap();
            let result: Box<dyn Any> = match invocation.func_name {
                "add" => Box::new(a+b),
                "subtract" => Box::new(a-b),
                _ => Box::new(0)
             };
            invocation.return_value = Some(result);
        }
    }
    
    #[dynamic_proxy(Interceptor)]
    pub trait MyTrait {
        fn add(self, a: i32, b: i32) -> i32;
        fn subtract(self, a: i32, b: i32) -> i32;
    }

    #[test]
    fn add() {
        use crate::tests::Interceptor;
        // use crate::tests::MyTrait;
        let s = Interceptor {};
        assert_eq!(s.add(6, 7), 13);
    }
    #[test]
    fn subtract() {
        use crate::tests::Interceptor;
        // use crate::tests::MyTrait;
        let s = Interceptor {};
        assert_eq!(s.subtract(8, 3), 5);
    }

}
