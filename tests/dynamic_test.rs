#[cfg(test)]
mod tests {
    use dynamic_proxy::dynamic_proxy;
    use dynamic_proxy_types::{DynamicProxy, InvocationInfo};

    pub struct Interceptor;

    impl DynamicProxy for Interceptor {
        fn call<T>(self: &Self, invocation: InvocationInfo) -> usize  {
            invocation.func_name.len()
        }
    }

    #[dynamic_proxy(Interceptor)]
    pub trait MyTrait {
        fn add(self: Self, a: i32, b:i32) -> i32;
    }

    #[test]
    fn function_name() {
        let s = Interceptor {};
        assert_eq!(s.add(6, 7), 3);
    }
}




