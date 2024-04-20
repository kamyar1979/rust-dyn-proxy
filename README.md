# Dynamic Proxy for Rust!

## Preface

### Understanding Proxies

In software engineering, a proxy is a placeholder or surrogate that controls access 
to another object or service. Proxies are used to add a layer of indirection to support 
distributed, controlled, or lazy access. They can also be used for logging, security, 
caching, and other purposes.

### What about dynamic proxy?

Dynamic proxies are proxies that are created dynamically at runtime. They allow you to create a proxy object
without explicitly writing a concrete proxy class. This is typically achieved through reflection or by using
a proxy generation library. Dynamic proxies are especially useful in scenarios where you need to intercept
method calls to add behavior, such as logging, profiling, or security checks, without modifying the original class.

Note the following simple Rust code:

```rust
trait Calculator {
    fn add(&self, a: i32, b: i32) -> i32;
    fn subtract(&self, a: i32, b: i32) -> i32;
}

struct CalculatorImpl;

impl Calculator for CalculatorImpl {
    fn add(&self, a: i32, b: i32) {
        a+b
    }
    fn subtract(&self, a: i32, b: i32) {
        a-b
    }
}
```

The trait and the implementation both live the same place. What if we need to invoke some code within another
process or simply parse the function name and parameters and create the result? For instance, suppose we want
to run some SQL functions. Normally we have to create a trait containing all the functions and write code to
send parameters and parse the result. BUt the code contains too much boilerplate! We feel repeating the same
code many times. What if we could create a general function to create the sql command, add the parameters and
parse the result into Rust types? We need to call the function with the name of the SQL function and arguments
and return back the SQL given result. Here is where Dynamic Proxy shines.

### Our implementation

Since Rust is a very strict languages, we have nothing like Java/.NEt reflection, or Python double-underscore 'call' 
method. The only way of implementing something similar is to use powerful procedural macros. They expand prior to 
compilation into pure Rust code. We have implemented an attribute macro which gets the empty struct name. 


```rust
pub struct Interceptor;

    impl DynamicProxy for Interceptor {
        fn call(&self, invocation: &mut InvocationInfo){
            let a = invocation.arg_values[0].downcast_ref::<i32>().unwrap();
            let b = invocation.arg_values[1].downcast_ref::<i32>().unwrap();
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
```    

The macro 'dynamic_proxy' gets the name of the struct we want to be implemented for trait, and then implements all the
trait items in which forwards the invocation to the 'call' function which is a member of DynamicProxy trait. There is an
struct containing the function signature and the argument values.

```rust
pub struct InvocationInfo<'a> {
    pub func_name: &'a str,
    pub arg_names: &'a[&'a str],
    pub arg_values: &'a [Box<dyn Any>],
    pub return_type: TypeId,
    pub return_value: Option<Box<dyn Any>>
}
```

The consumer must fill the return_value with the function response. The test code would be clear enough.

