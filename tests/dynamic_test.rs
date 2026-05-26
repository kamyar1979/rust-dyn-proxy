
#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use dynamic_proxy::{AsyncDynamicProxy, DynamicProxy, InvocationInfo, dynamic_proxy};

    pub struct Interceptor;

    pub struct GenericInterceptor;

    pub struct NonCloneResult {
        value: i32,
    }

    impl DynamicProxy for Interceptor {
        fn call(&self, invocation: &mut InvocationInfo) {
            let a = invocation.get_arg_value::<i32>(0);
            let b = invocation.get_arg_value::<i32>(1);
            assert_eq!(invocation.arg_names[0], "a");
            assert_eq!(invocation.arg_names[1], "b");
            assert_eq!(invocation.get_arg_type(1), TypeId::of::<i32>());
            let expected_return_type = match invocation.func_name {
                "notify" => TypeId::of::<()>(),
                "non_clone" => TypeId::of::<NonCloneResult>(),
                _ => TypeId::of::<i32>(),
            };
            assert_eq!(invocation.return_type, expected_return_type);
            match invocation.func_name {
                "non_clone" => invocation.set_return_value(NonCloneResult { value: a + b }),
                _ => invocation.set_return_value(
                    match invocation.func_name {
                        "add" => a + b,
                        "subtract" => a - b,
                        _ => 0
                    }),
            }
        }
    }

    impl AsyncDynamicProxy for Interceptor {
        async fn call_async(&self, invocation: &mut InvocationInfo<'_>) {
            let a = invocation.get_arg_value::<i32>(0);
            let b = invocation.get_arg_value::<i32>(1);
            assert_eq!(invocation.arg_names[0], "a");
            assert_eq!(invocation.arg_names[1], "b");
            assert_eq!(invocation.get_arg_type(1), TypeId::of::<i32>());
            let expected_return_type = match invocation.func_name {
                "notify_async" => TypeId::of::<()>(),
                "non_clone_async" => TypeId::of::<NonCloneResult>(),
                _ => TypeId::of::<i32>(),
            };
            assert_eq!(invocation.return_type, expected_return_type);
            match invocation.func_name {
                "non_clone_async" => invocation.set_return_value(NonCloneResult { value: a + b }),
                _ => invocation.set_return_value(
                    match invocation.func_name {
                        "add" => a + b,
                        "subtract" => a - b,
                        "add_async" => a + b,
                        _ => 0
                    }),
            }
        }
    }

    impl DynamicProxy for GenericInterceptor {
        fn call(&self, invocation: &mut InvocationInfo) {
            assert_eq!(invocation.func_name, "identity");
            assert_eq!(invocation.arg_names[0], "value");
            assert_eq!(invocation.return_type, TypeId::of::<String>());
            invocation.set_return_value(invocation.get_arg_value::<String>(0).clone());
        }
    }

    #[dynamic_proxy(Interceptor)]
    pub trait MyTrait {
        fn add(self, a: i32, b: i32) -> i32;
        fn subtract(self, a: i32, b: i32) -> i32;
        fn notify(self, a: i32, b: i32);
        fn non_clone(self, a: i32, b: i32) -> NonCloneResult;
        async fn add_async(self, a: i32, b: i32) -> i32;
        async fn notify_async(self, a: i32, b: i32);
        async fn non_clone_async(self, a: i32, b: i32) -> NonCloneResult;
    }

    #[dynamic_proxy(GenericInterceptor)]
    pub trait GenericTrait<T: 'static + Send + Sync> {
        fn identity(self, value: T) -> T;
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

    #[test]
    fn notify() {
        use crate::tests::Interceptor;
        let s = Interceptor {};
        s.notify(1, 2);
    }

    #[tokio::test]
    async fn notify_async() {
        use crate::tests::Interceptor;
        let s = Interceptor {};
        s.notify_async(1, 2).await;
    }

    #[test]
    fn non_clone() {
        use crate::tests::Interceptor;
        let s = Interceptor {};
        assert_eq!(s.non_clone(6, 7).value, 13);
    }

    #[tokio::test]
    async fn non_clone_async() {
        use crate::tests::Interceptor;
        let s = Interceptor {};
        assert_eq!(s.non_clone_async(6, 7).await.value, 13);
    }

    #[test]
    fn generic_trait() {
        use crate::tests::GenericInterceptor;
        let s = GenericInterceptor {};
        assert_eq!(s.identity(String::from("hello")), "hello");
    }
}
