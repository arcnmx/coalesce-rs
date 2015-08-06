//! Coalesce allows you to unify disjoint types on the stack.
//!
//! It is often useful to return different implementations of a common trait such as
//! `Iterator` or `Read` from conditional branches. The `coalesce!` macro makes it
//! easy to unify them into a common trait object:
//!
//! ```
//! #[macro_use]
//! extern crate coalesce;
//!
//! use coalesce::Coalesce2;
//! use std::iter::repeat;
//!
//! # fn main() {
//! # fn some_condition() -> bool { true }
//! let mut i = if some_condition() {
//!     Coalesce2::A(repeat(5u32).take(2))
//! } else {
//!     Coalesce2::B(0u32..8)
//! };
//! let i = coalesce!(2 => |ref mut i| i as &mut Iterator<Item=u32>);
//!
//! for x in i {
//!     println!("{}", x);
//! }
//! # }
//! ```

macro_rules! coalesce_ {
    ($i:ident: $($v:ident: $t:ident),*) => {
        pub enum $i<$($t),*> {
            $($v($t)),*
        }
    };
}

coalesce_!(Coalesce0: );
coalesce_!(Coalesce1: A:A);
coalesce_!(Coalesce2: A:A, B:B);
coalesce_!(Coalesce3: A:A, B:B, C:C);
coalesce_!(Coalesce4: A:A, B:B, C:C, D:D);
coalesce_!(Coalesce5: A:A, B:B, C:C, D:D, E:E);
coalesce_!(Coalesce6: A:A, B:B, C:C, D:D, E:E, F:F);
coalesce_!(Coalesce7: A:A, B:B, C:C, D:D, E:E, F:F, G:G);
coalesce_!(Coalesce8: A:A, B:B, C:C, D:D, E:E, F:F, G:G, H:H);
coalesce_!(Coalesce9: A:A, B:B, C:C, D:D, E:E, F:F, G:G, H:H, I:I);

/// Coalesces multiple values into one common (often borrowed) type.
///
/// See the crate documentation for an example.
#[macro_export]
macro_rules! coalesce {
    (:&mut $i:expr; $t:ident $id:ident $x:expr => $($v:ident)*) => {
        match $i {
            $($crate::$t::$v(ref mut v) => { let $id = v; $x }),*
        }
    };
    (:&$i:expr; $t:ident $id:ident $x:expr => $($v:ident)*) => {
        match $i {
            $($crate::$t::$v(ref v) => { let $id = v; $x }),*
        }
    };
    (:$i:expr; $t:ident $id:ident $x:expr => $($v:ident)*) => {
        match $i {
            $($crate::$t::$v(v) => { let $id = v; $x }),*
        }
    };

    (0 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce0 $id $x => ) };
    (1 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce1 $id $x => A) };
    (2 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce2 $id $x => A B) };
    (3 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce3 $id $x => A B C) };
    (4 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce4 $id $x => A B C D) };
    (5 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce5 $id $x => A B C D E) };
    (6 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce6 $id $x => A B C D E F) };
    (7 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce7 $id $x => A B C D E F G) };
    (8 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce8 $id $x => A B C D E F G H) };
    (9 => |ref mut $id:ident| $x:expr) => { coalesce!(:&mut $id; Coalesce9 $id $x => A B C D E F G H I) };

    (0 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce0 $id $x => ) };
    (1 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce1 $id $x => A) };
    (2 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce2 $id $x => A B) };
    (3 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce3 $id $x => A B C) };
    (4 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce4 $id $x => A B C D) };
    (5 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce5 $id $x => A B C D E) };
    (6 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce6 $id $x => A B C D E F) };
    (7 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce7 $id $x => A B C D E F G) };
    (8 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce8 $id $x => A B C D E F G H) };
    (9 => |ref $id:ident| $x:expr) => { coalesce!(:&$id; Coalesce9 $id $x => A B C D E F G H I) };

    (0 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce0 $id $x => ) };
    (1 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce1 $id $x => A) };
    (2 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce2 $id $x => A B) };
    (3 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce3 $id $x => A B C) };
    (4 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce4 $id $x => A B C D) };
    (5 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce5 $id $x => A B C D E) };
    (6 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce6 $id $x => A B C D E F) };
    (7 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce7 $id $x => A B C D E F G) };
    (8 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce8 $id $x => A B C D E F G H) };
    (9 => |$id:ident| $x:expr) => { coalesce!(:$id; Coalesce9 $id $x => A B C D E F G H I) };
}

#[test]
fn test() {
    trait T { }
    struct S1; struct S2; struct S3;
    impl T for S1 { } impl T for S2 { } impl T for S3 { }
    let v = match 3 {
        0 => Coalesce3::A(S1),
        1 => Coalesce3::B(S2),
        _ => Coalesce3::C(S3),
    };

    fn assert_t(_: &T) { }
    assert_t(coalesce!(3 => |ref v| v as &_));

    let v = if false { Coalesce2::A(0u16) } else { Coalesce2::B(0u32) };
    assert_eq!(1u64, coalesce!(2 => |v| (v + 1) as u64));
}
