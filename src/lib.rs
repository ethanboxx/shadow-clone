#![no_std]
//! Go the the [readme](https://crates.io/crates/shadow-clone) file for more documentation.

/// Use this macro to clone variables into the current scope shadowing old ones.
///
/// # Examples
/// ```rust,compile_fail
/// let s = "foo".to_string();
/// let c = move |x: i32| format!("{}{}", s, x);
/// let bar = s;
/// ```
/// This will not compile as `s` has been moved into the closure.
///
/// This issue can be solved with this macro.
/// ```rust
/// use shadow_clone::shadow_clone;
/// let s = "foo".to_string();
/// {
///     shadow_clone!(s);
///     let c = move |x: i32| format!("{}{}", s, x);
/// }
/// let bar = s;
/// ```
/// That expands to,
/// ```rust
/// use shadow_clone::shadow_clone;
/// let s = "foo".to_string();
/// {
///     let s = s.clone();
///     let c = move |x: i32| format!("{}{}", s, x);
/// }
/// let bar = s;
/// ```
/// You can also clone multiple variables separated by commas. `shadow_clone!(foo, bar);`
///
/// You can also bind a clone as mutable by prefixing with `mut`. `shadow_clone!(mut foo);`
#[macro_export]
macro_rules! shadow_clone {
    { $to_clone:ident, $($tt:tt)* } => {
        $crate::shadow_clone!($to_clone);
        $crate::shadow_clone!($($tt)*)
    };
    { mut $to_clone:ident, $($tt:tt)* } => {
        $crate::shadow_clone!(mut $to_clone);
        $crate::shadow_clone!($($tt)*)
    };
    { (mut) $to_clone:ident, $($tt:tt)* } => {
        $crate::shadow_clone!(mut $to_clone);
        $crate::shadow_clone!($($tt)*)
    };
    { $to_clone:ident } => {
        let $to_clone = $to_clone.clone();
    };
    { mut $to_clone:ident } => {
        let mut $to_clone = $to_clone.clone();
    };
    { (mut) $to_clone:ident } => {
        $crate::shadow_clone!(mut $to_clone)
    };
    () => ()
}
