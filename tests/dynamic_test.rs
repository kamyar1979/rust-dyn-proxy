#[cfg(test)]
mod tests {
    use dynamic_proxy::dynamic_proxy;
    use dynamic_proxy_types::{DynamicProxy, InvocationInfo};
    // use crate::MyTrait;

    pub struct Interceptor;

    impl DynamicProxy for Interceptor {
        fn call(self: &Self, invocation: InvocationInfo) -> usize {
            invocation.func_name.len()
        }
    }


    #[dynamic_proxy(Interceptor)]
    pub trait MyTrait {
        fn add(self, a: i32, b: i32) -> i32;
        fn subtract(self, a: i32, b: i32) -> i32;
    }

    #[test]
    fn function_name() {
        use crate::tests::Interceptor;
        // use crate::tests::MyTrait;
        let s = Interceptor {};
        assert_eq!(s.subtract(6, 7), 8);
    }
}
