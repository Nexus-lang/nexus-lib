# Getting started

This is the perfect guide if you have no or very little prior programming experience. So let's go!

## Setting everything up

After you have finished the [setup](setup.md), create your first nexus script file in your IDE.

Note: The file has to end with .nx

Now we can finally start writing code! An important notice though: Especially in more complex examples, I advise you to type the code by hand and not some keyboard shortcuts.

## Writing the code

### Hello World

The first program will be one that simply prints "Hello, World!" or any text you want actually!

```go
print("Hello, World!")
```

> The text in quotation marks is called a string and can represent any text.

**Challenge:** Change the program to print `Hi Mom`

### Variables

You might already know the concept of variables from maths. But if you don't, here is a short explanation:

#### The concept of variables

A variable is an identifier that stands for a value.

Let's say you are writing an email with the same layout to a lot of people.

> Hello, `name`.
>
> With this email I want to to inform you
> ...

As you can see `name` is highlighted because it is the name of our variable. This variable stores the actual name. Let's represent the variable like this: `name = "John"`

#### Variables in Nexus

Let's expand our program like this:

```go
var name = "John"

print("Hello, {name}")
```

In nexus we define variables with the `var` keyword and then assign a value to it like you can see above.

The next thing you might notice is that we have changed the print(...) function. This change allows us to print out the value of the `name` variable. When you see curly brackets (`{...}`) in a String (String = sequence of words encased by quotation marks) most of the time this means that we want to use the value of the variable in the String.

**IMPORTANT:** Variable names are only allowed to have letters `abc...z`, `numbers` (not at the beginning tho) and underscores `_`

**Challenge:** Print your own name and change the variable name to be `my_name`

> **Tip:** You also need to change the variable name that is used in the `print` function

#### Other (variable) types

We have already looked at the String (String = sequence of words encased by quotation marks) type, but Nexus supports a few more types! Let's take a look at them.

**Strings (again):**

We have already looked at this type but here is a reminder:

```go
var name = "John"
```

**Numbers:**

This can be any number between 0 and infinite

```go
var age = 9
```

Numbers can also be represented more precisely with a floating point

```go
var age = 0.9123
```

**Booleans:**

Although the name sounds complex, booleans are very simple. They can represent two values: `true` or `false`

```go
var is_cool = true

var has_skill_issue = false
```

## The End (for now)

The next chapter will cover how to use handle variables even better [Advanced-variables](advanced_variables.md)
