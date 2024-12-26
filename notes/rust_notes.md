# Rust Notes

## Conversion between data structures

The Rust standard library provides two traits to convert data between various types. They are the From and Into traits:

```rust
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    // Required method
    fn into(self) -> T;
}
```
- Simple example

```rust
struct A {
    member: String,
}

struct B {
    value: String,
}

impl From<A> for B {
    fn from(source: A) -> Self {
        Self {
            value: source.member,
        }
    }
}

fn main() {
    let a = A { member: String::from("something") };
    let b = B::from(a);
    
    println!("{}", b.value);
}

```

- Here we convert data from struct `A` into struct `B`. 
- The `Into` trait is simply the inverse of the `From`, 
- so if we implement `From<A>` for `B` 
- then we can call `let b: B = a.into();` too:

```rust
fn main() {
    let a = A { member: String::from("something") };
    let b: B = a.into();
    
    println!("{}", b.value);
}
```

- This does not seem to be so useful, 
- but in reality there are a lot of cases in web service development when we have to convert similar data structures into each other. 
- Assume for example, 
- that we receive data from a `backend service` 
- and that data can be `deserialized` into structure `A`, 
- but we have to return a `JSON` to our client 
- and that `JSON` can be serialized only from structure `B`. 
- We can do this conversion easily with a `From` implementation.

- The `From` trait has no way to return an `error`. 
- If the conversion can fail, 
- we must use `TryFrom` instead:

```rust
pub trait TryFrom<T>: Sized {
    type Error;

    // Required method
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```
- This version returns a `Result` with two potential outcomes: 
- an `Ok` with the result of a successful conversion 
- or an `Err` with and error. 
- The associated type `Error` specifies the exact error type. 

A simple example:
```rust
use std::num::ParseIntError;

struct Number {
    value: i32,
}

impl TryFrom<String> for Number {
    type Error = ParseIntError;
    
    fn try_from(source: String) -> Result<Self, Self::Error> {
        Ok(Number { value: source.parse()? })
    }
}

fn main() {
    match Number::try_from(String::from("42")) {
        Ok(n) => {
            println!("{}", n.value);
        },
        Err(e) => {
            println!("Conversion failed {:?}", e);
        }
    }
}
```
- Here the potential `error` from `source.parse()` is a `ParseIntError` 
- so we have to specify it as the `Error` associated type.

- In the above example, 
- `42` can be converted into an `i32` value, 
- so our `TryFrom` implementation succeeds, 
- but replace `42` with `notanumber` and you will get a `ParseIntError`:

