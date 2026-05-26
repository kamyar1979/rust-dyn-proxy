# Dynamic Proxy for Rust

`dynamic-proxy` provides an attribute macro for turning a trait into calls to a
single interceptor object. It is useful when trait methods describe operations
that are handled dynamically, such as RPC calls, command dispatch, SQL
function calls, logging, profiling, or other proxy-style behavior.

The macro keeps the trait as the public API, then generates an implementation
for the interceptor type passed to `#[dynamic_proxy(...)]`.

## Example

```rust
use std::any::TypeId;
use dynamic_proxy::{dynamic_proxy, DynamicProxy, InvocationInfo};

pub struct Interceptor;

impl DynamicProxy for Interceptor {
    fn call(&self, invocation: &mut InvocationInfo) {
        let a = invocation.get_arg_value::<i32>(0);
        let b = invocation.get_arg_value::<i32>(1);

        assert_eq!(invocation.arg_names, ["a", "b"]);
        assert_eq!(invocation.get_arg_type(0), TypeId::of::<i32>());
        assert_eq!(invocation.return_type, TypeId::of::<i32>());

        let result = match invocation.func_name {
            "add" => a + b,
            "subtract" => a - b,
            _ => 0,
        };

        invocation.set_return_value(result);
    }
}

#[dynamic_proxy(Interceptor)]
pub trait Calculator {
    fn add(self, a: i32, b: i32) -> i32;
    fn subtract(self, a: i32, b: i32) -> i32;
}

let proxy = Interceptor;
assert_eq!(proxy.add(6, 7), 13);
assert_eq!(proxy.subtract(8, 3), 5);
```

## InvocationInfo

Every proxied call is represented as an `InvocationInfo`:

```rust
pub struct InvocationInfo<'a> {
    pub func_name: &'a str,
    pub arg_names: &'a [&'a str],
    pub arg_values: &'a [Box<dyn Any + Send + Sync>],
    pub return_type: TypeId,
    pub return_value: Option<Box<dyn Any + Send + Sync>>,
}
```

The interceptor reads argument values with `get_arg_value`, checks their
runtime types with `get_arg_type`, and sets a result with `set_return_value`.
For methods returning `()`, no return value is required.

## Async Methods

Async trait methods are forwarded through `AsyncDynamicProxy`:

```rust
use dynamic_proxy::{AsyncDynamicProxy, InvocationInfo};

impl AsyncDynamicProxy for Interceptor {
    async fn call_async(&self, invocation: &mut InvocationInfo<'_>) {
        let a = invocation.get_arg_value::<i32>(0);
        let b = invocation.get_arg_value::<i32>(1);
        invocation.set_return_value(a + b);
    }
}

#[dynamic_proxy(Interceptor)]
pub trait AsyncCalculator {
    async fn add_async(self, a: i32, b: i32) -> i32;
}
```

## RPC-Oriented Contract

This crate is designed around owned invocation payloads. Arguments and return
values must be:

- `'static`
- `Send`
- `Sync`

That matches RPC-style use cases where values need to be serialized, queued,
sent across threads, or moved beyond the caller's stack frame.

Use owned values instead of borrowed values:

- Use `String` instead of `&str`.
- Use `Vec<T>` instead of `&[T]`.
- Use owned request/response structs instead of structs containing borrowed
  fields.

Borrowed arguments and borrowed return values are intentionally rejected by the
macro with a compile-time error.

## Supported Trait Shapes

The macro supports:

- Sync methods through `DynamicProxy`.
- Async methods through `AsyncDynamicProxy`.
- Unit-returning methods.
- Non-`Clone` return values.
- Trait generics, lifetimes, and where clauses.
- Identifier arguments like `value: String`.

The macro rejects:

- Borrowed arguments and borrowed return values.
- Destructured or non-identifier arguments.
- Required associated constants.
- Required associated types.

Associated items with defaults may remain on the trait, but the macro does not
generate custom associated item implementations.

## Generic Trait Example

```rust
use dynamic_proxy::{dynamic_proxy, DynamicProxy, InvocationInfo};

pub struct GenericInterceptor;

impl DynamicProxy for GenericInterceptor {
    fn call(&self, invocation: &mut InvocationInfo) {
        invocation.set_return_value(
            invocation.get_arg_value::<String>(0).clone()
        );
    }
}

#[dynamic_proxy(GenericInterceptor)]
pub trait Service<T: 'static + Send + Sync> {
    fn identity(self, value: T) -> T;
}

let proxy = GenericInterceptor;
assert_eq!(proxy.identity(String::from("hello")), "hello");
```

## Notes

For non-unit return values, the interceptor must set `return_value` to the
exact return type expected by the trait method. If the value is missing or has
the wrong type, the generated method will panic while unwrapping/downcasting
the result.
