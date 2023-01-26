/*
Functions as values are MAJOR in Rust!, a 'closure' is an object that acts as a function, which implements
'fn', 'Fn', 'FnMut' and 'FnOnce' traits. These traits are automatically implemented if permitted.

// Functions can be sent as parameters and these are called Higher-Order functions, The Trait bound 'Fn' is a closure.
The square operation is applied using an inline closure definition sent to the map function of the
iterator.

It is possible to define functions or methods that accept closures as arguments. To use the
closure as a callable function, a bound of Fn, FnMut, or FnOnce must be specified.
Here is a HoF definition accepting a function g and an argument x. The definition
constrains g and x to process u32 types, and defines some mathematical operations
involving calls to g.
*/

fn f<T>(g: T, x: u32) -> u32 where T: Fn(u32) -> u32 {
    g(x + 1) * g(x + 2)
}
fn main() {
    (0..10).map(|x| x * x);

// Closure with a complex body

(0..10).map(|x| {
    fn f(y: u32) -> u32 {
    y * y
}
let z = f(x + 1) * f(x + 2); 
});


 

 println!("{:#?}", f(|x| {x * x}, 2)); // f(1 + 1) * f(1 + 2) squared = 36, f(2 + 1) * f(2 + 2) squared = 144

 /*
 Here is an iterator from 0 to 10 followed by many chained iterator combinators.
The map function returns a new value from an original. 
inspect looks at a value, does not change it, but permits side-effects. 
filter omits all values that do not satisfy a predicate.
filter_map filters and maps with a single function. 
The fold reduces all results to a single value, starting from an initial value, working left to right. 
*/

(0..10).map(|x| x * x)
.inspect(|x|{ println!("value {}", *x) })
.filter(|x| *x<3)
.filter_map(|x| Some(x))
.fold(0, |x,y| x+y);
}



