use crate::tests::MyTrait;

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use dynamic_proxy::{DynamicProxy, InvocationInfo, dynamic_proxy};
    use std::ops::Deref;
    use dynamic_proxy_types::AsyncDynamicProxy;
    use tokio::spawn;
    // use crate::MyTrait;

    pub struct Interceptor;

    impl DynamicProxy for Interceptor {
        fn call(&self, invocation: &mut InvocationInfo) {
            let a = invocation.get_arg_value::<i32>(0);
            let b = invocation.get_arg_value::<i32>(1);
            assert_eq!(invocation.arg_names[0], "a");
            assert_eq!(invocation.arg_names[1], "b");
            assert_eq!(invocation.get_arg_type(1), TypeId::of::<i32>());
            assert_eq!(invocation.return_type, TypeId::of::<i32>());
            invocation.set_return_value(
                match invocation.func_name {
                    "add" => a + b,
                    "subtract" => a - b,
                    _ => 0
                })
        }
    }
    
    impl AsyncDynamicProxy for Interceptor {
        async fn call_async(&self, invocation: &mut InvocationInfo<'_>) {
            let a = invocation.get_arg_value::<i32>(0);
            let b = invocation.get_arg_value::<i32>(1);
            assert_eq!(invocation.arg_names[0], "a");
            assert_eq!(invocation.arg_names[1], "b");
            assert_eq!(invocation.get_arg_type(1), TypeId::of::<i32>());
            assert_eq!(invocation.return_type, TypeId::of::<i32>());
            invocation.set_return_value(
                match invocation.func_name {
                    "add" => a + b,
                    "subtract" => a - b,
                    "add_async" => a + b,
                    _ => 0
                })
        }
    }

    #[dynamic_proxy(Interceptor)]
    pub trait MyTrait {
        fn add(self, a: i32, b: i32) -> i32;
        fn subtract(self, a: i32, b: i32) -> i32;
        async fn add_async(self, a: i32, b: i32) -> i32;
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

    #[tokio::test]
    async fn add_async() {
        use crate::tests::Interceptor;
        // use crate::tests::MyTrait;
        let s = Interceptor {};
        assert_eq!(s.add_async(6, 7).await, 13);
    }
}


