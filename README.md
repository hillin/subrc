# Subrc

![crates.io](https://img.shields.io/crates/v/subrc.svg)
![docs.rs](https://img.shields.io/docsrs/subrc/latest)

A tiny crate that exposes a `Rc` like struct, which can be used to create a reference counted pointer to a subregion (member, or member of member etc.) of a `Rc`.

## Example

```rust
struct Foo {
    value: i32,
}

let rc = Rc::new(Foo { value: 42 });
let subrc = Subrc::new(rc.clone(), |foo| &foo.value);
// or 
let subrc = subrc!(rc.value);

// subrc derefs to 42
assert_eq!(*subrc, 42);
// subrc points to rc.value
assert!(std::ptr::eq(&*subrc, &rc.value));
```