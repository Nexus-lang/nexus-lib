# Functions

In this third tutorial, we will learn how to use functions.

You might already know about functions in mathematics but we will still elaborate the topic a bit further before diving in to the code

## The theory behind functions

Like variables can be used to store values, functions are used to store a sequence of functions and variables.

Like most topics it will take some time before you fully understand how functions work

## Functions in Nexus

As previously stated, this will be a learning by doing experience so let's just dive into the code

```go
// Let's start by defining a function
// like almost everything, functions are stored using variables
var my_function = func() {
    print("Hello world")
}
```

Let's break down what we just did. First we defined a variable (`var my_function = ...`) that will be used as a reference to the function

Then we created the actual function using the `func` keyword and braces behind it `()` Then we use curly brackets `{...}` in which we define the behaviour of our function.

In the curly bracket block, for the moment we will just write a simple `print(...)` function that prints out `"Hello World"`

When you run the code now, you might realize that nothing happens. This is because we only define the function but never actually call it.

Here is an example on how to do so:

```go
var my_function = func() {
    print("Hello world")
}

// Here we call out function
my_function()
```

And it should output `Hello world`.

[WIP]
