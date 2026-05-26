use dynamic_proxy::{dynamic_proxy, DynamicProxy, InvocationInfo};

struct Interceptor;

impl DynamicProxy for Interceptor {
    fn call(&self, _invocation: &mut InvocationInfo) {}
}

#[dynamic_proxy(Interceptor)]
trait PatternArg {
    fn call(self, (a, b): (i32, i32));
}

fn main() {}
