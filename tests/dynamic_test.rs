use dynamic_proxy_types::*;
use dynamic_proxy::dynamic_proxy;


struct Interceptor;

impl DynamicProxy for Interceptor {
    fn call<T>(invocation: InvocationInfo) {
        todo!()
    }
}

#[dynamic_proxy(Interceptor)]
trait MyTrait {
    fn add(a: i32, b:i32) -> i32;
}


