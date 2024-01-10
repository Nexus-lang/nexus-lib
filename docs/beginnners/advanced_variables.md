# Advanced Variables

This guide will teach you more about variables and data types.

## The difference between `const` and `var`

### Mutable variables

In nexus, there are two ways to define a variable. The first way, we have already talked about is by using `var`.

`var` enables you to define a variable of which you can modify the value or even reassign the variable entirely.

Let's look at this for a quick example:

```go
var x = 0 // We assign a value to the variable x

x = 2 // Here we give the variable x a new value

print(x) // And here we print the variables value
```

This will output the value `2` as we reassigned `x` to this value before outputting it.

As you already saw you can reassign variables by using their name (`x`), `=` and the new value (`2`)

### Constant variables

Especially in larger projects, you sometimes want variables that cannot be modified to prevent unwanted behaviour (bugs)

To deal with this problem, nexus has the `const` keyword.

Let's look at this for a quick example quick example:

```go
const y = 0 // We assign 0 to the variable y

y = 2 // We try to reassign this value, but it throws an error. Because it is constant
```

As you can see it is not possible to reassign this variable because it will throw an error.
We can still use this like any other variable however.

## Type annotations

Another common cause of bugs are type modifications.

For example:
```go
// We define the variable name and give it the value "John". As we learned in the previous tutorial, this variables's datatype is called string and it always has to be in quotation marks.
var name = "John"

// We mistake this variable for the age and give it the value 45
name = 45

// When we try to print the name it instead gives us the age (45) because we reassigned the variable name to the value of the age (45)
print(name)
```

To avoid this behaviour we can make it so the variable can only hold specific values.

Look at this example for how to do it:
```go
// We define a variable that can only be a string
var name: Str = "John"

// When we try to reassign the variable to a number it will throw an errror because the value can only be of type String
name = 45
```

Ok, so we learned that we can give variables explicit types using a colon after the name and then give it the correct data type.

```go
var name: Str = "Thomas"
```

Valid data types at the moment are: `Str` for strings, `Num` for numbers, `Bool` for booleans. Later on you will also be able to create your own data types 👀

## Quick assigning

Typing out `var` or `const` every time can be annoying. That's why we have introduced an easy way to assign variables.

Let's look at this example once again:

```go
var x = 0

// This is the same as the first line but shorter:
x := 0
```

The same works for constants:

```go
const y = 0

// This again is the same as the first line but shorter:
y :: 0
```

We now know how to quickly assign variables. Amazing!

## The end (for now)

Great, you now know everything about variables and constants. In the next tutorial we will take a look at one of the most important features of any programming language: [Functions](functions.md)
