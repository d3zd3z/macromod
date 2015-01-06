Simple demonstration of macro non-hygiene in Rust

This demonstrates a couple places where hygiene is lacking with macros
as implemented currently in Rust.

*   Type references use whatever name is in scope:

    The `thing!()` macro builds a `Thing`, expecting it to have the
    type in scope, and be the correct type.  The line marked 'good1'
    in `main.rs` is necessary to be able to have this invocation work
    in the main program.  If, this is removed, and replaced with
    `bad1`, the macro expansion will fail.  The macro will reference
    the locally defined type instead of the one defined in the scope
    of the macro, and will fail because the field name is incorrect.

*   Function names have similar problems, as can be seen by the `blop`
    function and the `see_blop!()` macro that expands to a call to it.

Using a full path to either the type or function name doesn't work,
because there is no consistent path that can refer to this function.
For example, in the library, `::stuff::Thing` would work, but in
another module, it would likely need to be `macros::stuff::Thing`.
Even then, the name could change because of an `extern crate macros as
...` declaration.
