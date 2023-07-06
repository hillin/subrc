use std::{marker::PhantomData, ops::Deref, rc::Rc};

/**
* A reference counted pointer to a sub-region (member) of a [`Rc`].
*
* # Example
```rust
struct Foo {
    value: i32,
}

let rc = Rc::new(Foo { value: 42 });
let subrc = Subrc::new(rc.clone(), |foo| &foo.value);
// subrc derefs to 42
assert_eq!(*subrc, 42);
// subrc points to rc.value
assert!(std::ptr::eq(&*subrc, &rc.value));
```
*/
#[derive(PartialEq, Clone)]
pub struct Subrc<T, U> {
    rc: Rc<T>,
    offset: usize,
    #[doc(hidden)]
    _u: PhantomData<U>,
}

impl<T, U> Subrc<T, U> {
    unsafe fn get_offset(t: &T, u: &U) -> usize {
        let t_ptr = t as *const T as usize;
        let u_ptr = u as *const U as usize;

        if u_ptr < t_ptr {
            panic!("getter did not return portion of the object");
        }

        let offset = u_ptr - t_ptr;
        if offset >= std::mem::size_of::<T>() {
            panic!("getter did not return portion of the object");
        }

        offset
    }

    /**
       Create a [`Subrc`] pointer, which points to a subregion of the specified [`Rc`].
       The `getter` function is used to specify the subregion. It must return a reference to a subregion
       of the [`Rc`]. Returning anything else will result in a panic.

       # Panics
       In the `getter` function, returning anything other than a reference to a subregion of the [`Rc`]
       will result in a panic.

       ## Example
       ```rust
           let s = String::from("hello");
           let rc = Rc::new(s);
           let subrc = Subrc::new(rc.clone(), |s| &123);   // panic here: `123` is totally unrelated to `s`!
       ```
    */
    pub fn new<F>(rc: Rc<T>, getter: F) -> Self
    where
        F: FnOnce(&T) -> &U,
    {
        let offset = unsafe { Self::get_offset(&*rc, getter(&rc)) };
        Subrc {
            rc,
            offset,
            _u: PhantomData,
        }
    }

    pub fn get(&self) -> &U {
        unsafe {
            let t_ptr = &*self.rc as *const T as *const u8;
            let u_ptr = t_ptr.add(self.offset);
            &*(u_ptr as *const U)
        }
    }
}

impl<T, U> Deref for Subrc<T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Subrc;

    struct Foo {
        _value: i32,
        bar: Bar,
    }

    struct Bar {
        value: i32,
    }

    #[test]
    fn test_subrc_struct_member() {
        let foo = Foo {
            _value: 42,
            bar: Bar { value: 24 },
        };

        let rc = Rc::new(foo);
        let subrc = Subrc::new(rc.clone(), |foo| &foo.bar);
        assert_eq!(subrc.value, 24);
        assert!(std::ptr::eq(&*subrc, &rc.bar));
    }

    #[test]
    #[should_panic]
    fn should_panic_for_invalid_offset() {
        let foo = Foo {
            _value: 42,
            bar: Bar { value: 24 },
        };

        let rc = Rc::new(foo);
        let _subrc = Subrc::new(rc.clone(), |_| &42);
    }
}
