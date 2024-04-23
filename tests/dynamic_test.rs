#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use dynamic_proxy::{DynamicProxy, InvocationInfo, dynamic_proxy};
    use std::ops::Deref;
    // use crate::MyTrait;

    pub struct Interceptor;

    impl DynamicProxy for Interceptor {
        fn call(&self, invocation: &mut InvocationInfo){
            let a = invocation.arg_values[0].downcast_ref::<i32>().unwrap();
            let b = invocation.arg_values[1].downcast_ref::<i32>().unwrap();
            assert_eq!(invocation.arg_names[0], "a");
            assert_eq!(invocation.arg_names[1], "b");
            assert_eq!(invocation.arg_values[1].deref().type_id(), TypeId::of::<i32>());
            assert_eq!(invocation.return_type, TypeId::of::<i32>());
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
