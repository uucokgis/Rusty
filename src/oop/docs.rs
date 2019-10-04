/*
There is not many technical details around OOP in Rust.
There is no inheritance

The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
When we use trait objects, Rust must use dynamic dispatch.

You can only make object-safe traits into trait objects. Some complex rules govern all the properties
that make a trait object safe, but in practice, only two rules are relevant. A trait is object safe
if all the methods defined in the trait have the following properties:

    The return type isn’t Self.
    There are no generic type parameters.

*/