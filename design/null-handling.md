# Null handling

## The Problem with null/none

Null or None as it is called in some programming languages, bears a huge risk: Using a null-value where a non-null-value is expected, leading to errors, crashes or even undefined behaviour.

However, not having a null/none type in many cases is as bad if not even worse than having one. Since the absense of this concept leads to people using others ways to display the absence of a value which due to not being handled as well as null can lead to the same bugs that are a lot harder to debug.

And if you don't have any ideas, just take **inspiration** from other languages :p

## Taking a look at other languages

Let's look at other languages and how they handle intializing a nullable value

### Kotlin

Let's look at this kotlin code to see a very simple example of using null

```kotlin
fun main() {
    var test: String? = "Hello"
    test = null
}
```

As you might have realised there is a question mark behind the type annotation which indicates that we want the variable to be able to represent `null`.

While this system is very intuitive, this is also one of its main flaws, sice you don't want null to be intuitive or people will use it without thinking about it twice, leading to the previously mentioned problems.

So let's take a look at the next language, `rust`!

### Rust

```rust
fn main() {
    let mut test: Option<&str> = Some("Hello");
    test = None;
}
```

As you might have realized, this is a lot more code than the kotlin example. And if you also looked at the type you can spot that unlike the previous example this is not actually a direct string but a wrapper around it.

While this is not as intuitive and simple as the kotlin example it also makes the user think twice if they actually want to/need to use null and also makes them aware of the potential null value beforehand.

### Conclusion

While I praised rust's implementation a lot, I think that in the context of a simple, interpreted, beginner-friendly language it does not make sense to use a complex system like that but rather take a closer look at kotlin.

## Null references

Before we actually take a look at how we ourselves can fix the issue, we also need to consider the other part of the puzzle: Referencing a value that potentially is a null value.

### Kotlin

While kotlin's solution looks a lot more distinct from rust, under the hood they basically work the same. You have a wrapper for the value and when you want to access it you have to consider that the value might be null. There are two effective ways to do so:

#### 1. The safe approach

We pass on the value to the next variable until we are ready to safely unwrap it

```kotlin
fun main() {
    var test: String? = null
    val firstChar: Char? = test?.get(0)
    //                         ^ this way we safely call the method
    if firstChar != null {
        // unwrap the value and do something with it
    }
}
```

#### 2. The unsafe approach

We just unwrap the value regardless of whether it might be null or not and risk throwing an error.

```kotlin
fun main() {
    var test: String? = "Test"
    var firstChar: Char = test!!.get(0)
    //                        ^^ this way we just unwrap the value  
}
```

### Rust

As previously stated both rust and kotlin's systems works similar. The biggest difference most likely is the difference in handling the nullable value.

#### 1. The safe approach

```rust
fn main() {
    let test: Option<&str> = Some("Hello");
    // Test if the value is none or not-none. Match statement is pretty much the same as kotlin and nexus' `when` statement
    match test {
        Some(val) => println("{}", val), // print the value if it is not-none
        None => (), // do nothing if the value is None
    }
}
```

#### 2. The unsafe approach

```rust
fn main() {
    let test: Option<&str> = Some("Hello");
    println!("{}", test.unwrap()); // We just use tha value and risk it being none. Luckily rust directly throws an error when trying to unwrap the value, before we can do even more damage
}
```

### Conclusion

Personally I think having a mix between kotlin and rust's safe none handling would be be best so we will go with something along those lines. As for the unsafe approach however, I prefer rust.


## Our own solution

```kotlin
const myVal? = none
#          ^ we use the question mark, similar to kotlin but without requiring the type

when myVal? {
    none -> (),
    else -> puts(myVal), # we can safely use myVal now and dont need to unwrap it
}
```
