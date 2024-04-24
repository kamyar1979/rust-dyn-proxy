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

The trait and its implementation usually reside in the same place. But what if we want to execute code 
in another process or dynamically parse function names and parameters to produce results?

Consider a scenario where we need to execute SQL functions. Typically, we create a trait 
containing all the functions and write code to send parameters and parse results. However, 
this approach often involves repetitive boilerplate code.

What if we could create a single, general function to construct SQL commands, add parameters, 
and parse results into Rust types? Imagine calling this function with the SQL function name 
and its arguments and receiving the corresponding result. This is where Dynamic Proxy shines.
It enables us to dynamically handle function calls, simplifying our code and reducing redundancy.



### Our implementation

Rust is known for its strictness, lacking features like Java or .NET's reflection or Python's 
`__call__` method. To achieve similar functionality, Rust offers powerful procedural macros. 
These macros expand before compilation into pure Rust code.

In our project, we've implemented an attribute macro that takes the name of an empty struct. 
This macro enables us to emulate behavior similar to reflection or dynamic method invocation 
found in other languages.


```rust
pub struct Interceptor;

impl DynamicProxy for Interceptor {
    fn call(&self, invocation: &mut InvocationInfo){
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

