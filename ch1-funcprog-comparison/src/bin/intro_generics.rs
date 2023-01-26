use std::process::Output;

struct PointU32 {
    x: u32,
    y: u32
}

struct PointF32 {
    x: f32,
    y: f32
}

struct PointI32 {
    x: i32,
    y: i32
}

// Simplify the above by adding Generics

struct Point<T> {
    x: T,
    y: T
}


// Functions are also problematic w/o generics, some generics have mandatory trait bounds to permit behaviors

fn foo_u32(x: u32) -> u32 {
    x * x
}

fn foo_f32(x: f32) -> f32 {
    x * x
}

fn foo_i32(x: i32) -> i32 {
    x * x
}

/* Generics with Trait bounds - in this case the for the Mul trait

std::ops::Mul
pub trait Mul<Rhs = Self> {
    type Output;

    fn mul(self, rhs: Rhs) -> Self::Output;
}
*/

fn foo<T>(x: T) -> T  where T: std::ops::Mul<Output=T> + Copy {
    x * x
   
}

// Functions can be sent as parameters and these are called Higher-Order functions, The Trait bound 'Fn' is a closure. See closures in the 'intro_functions.rs' file

fn bar<F,T> (f: F, x: T) -> T where F: Fn(T) ->T {
    f(x)
}




fn main() {}