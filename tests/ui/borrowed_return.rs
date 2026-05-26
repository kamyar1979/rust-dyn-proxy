use dynamic_proxy::{dynamic_proxy, DynamicProxy, InvocationInfo};

struct Interceptor;

impl DynamicProxy for Interceptor {
    fn call(&self, _invocation: &mut InvocationInfo) {}
}

#[dynamic_proxy(Interceptor)]
trait BorrowedReturn {
    fn call(self, value: String) -> &str;
}

fn main() {}
