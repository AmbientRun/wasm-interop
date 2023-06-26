# WebAssembly Interop

This example project demonstrates WebAssembly interop.

- loading and executing a wasm module from javascript, henceforth called `bootstrapping`

- Loading wasm from wasm and calling it

## Wasm to Wasm

When compiling a Rust library to WebAssembly, it is built like a `C` library.

The exported function can only take and return simple primitive types.

`String`, `structs`, `&` etc are not allowed.

`struct`s are rust-specific, and there is no concept of passing composite types to wasm. It is possible to pass some
composite types across the boundaries, as rust will `flatten` the struct fields into separate arguments, but on the
wire between wasm it is plain arguments. The struct support is only some sugar and convenience that the rust
compiler will provide. To make this work, `#[repr(C)]`is required to ensure `rustc` is not reordering fields and
adding padding.

`Strings` are similar, and have a pointer inside them to the Wasm modules memory. Passing it to JavaScript or
between modules require disassembly, and re-encoding and allocating on the receiver side.

Passing pointers "raw" between modules is dangerous since pointers have no memory identifier, and will always refer
to a byte offset in the current module. As such, passing a pointer from `A => B` which points to valid memory in `A`
will point to something completely different in the memory of `B`. As such, serialization is done.

`&` are not allowed since rusts lifetimes don't map over FFI. Fat slices are not allowed either.

# Wasm-bindgen

So, you may be wondering.

How does `wasm-bindgen` allow calling into Rust, and calling from Rust into Js using complicated types such as
`String`s, `struct`s, `&` etc?

The `#[wasm_bindgen]` proc macro generates the glue required that wraps your nice function with an _FFI-safe_
function that takes raw offsets and primitive types, and then assembles them together into rust types to call your nice Rust function.

For example

```rust
fn foo(name: String)
```

Gets converted into
The glue is generated to convert a raw byte address and a length into a rust UTF-8 string.

```rust

unsafe fn foo(name_ptr: *const u8, name_len: i32) {
    let name = slice::from_raw_parts(name_ptr, name_len);
    let name = String::from_utf8_unchecked(name);

    foo(name)
}
```

The function is then exported using the `C` abi as:

`fun(i32, i32)`

Notably, pointers are passed as integer values, I.e; 32-bit signed integers.

Calling this function requires allocating and copying your UTF-8 string data into the target modules memory, and then
using that pointer to call the function.

**The passed pointer must be from the memory of the module that is called**

Just passing your `str::as_ptr()`, or `char*` from your module to the target will result in undefined behavior as that pointer
now refers to something else. You need to copy it into the target memory before calling. A good way to think of it is as
calling between different system processes, which have a different address space.

Calling this function from _JavaScript_, requires taking your js `string` and encoding it into an `Uint8Array` and then
allocating and copying the bytes into the target module's memory to obtain a pointer. The pointer is then what is
ultimately passed into wasm.

## Links

- (Rust Type conversions)[https://rustwasm.github.io/docs/book/print.html]
- (WASI Marshalling)[https://rob-blackbourn.github.io/blog/webassembly/wasm/javascript/c/clang/wasi-sdk/marshalling/2020/07/02/wasi-marshalling.html]
- (FromWasmAbi)[https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.FromWasmAbi.html]
