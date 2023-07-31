# Nexus

- [Introduction](#introduction)

- [Installation](#installation)

- [Documentation](#documentation)

- [Contributors](#contributors)

## Introduction

Nexus is intended to be a smart, modern and powerful scripting language.

It was built for creating packages and extensions for [rust](https://www.rust-lang.org/) applications.

### Features

Similar to it's father language rust, Nexus has features from both oop and fp.

This includes polymorphism and in some limited cases inheritance.

As for functional programming it supports higher-order functions, mapping, annonymous functions

### Inspiration

The language takes inspiration from [go](https://go.dev) & [kotlin](https://kotlinlang.org/) as well as some features from [python](https://www.python.org/) and [rust](https://www.rust-lang.org/)

## Installation

If you want to try out Nexus you will need to compile it manually for now.

To do this follow these steps:

- Download rust here: <https://www.rust-lang.org/tools/install>

- Run the main.rs file by executing `cargo run` in the terminal

Currently text.nx contains the sample source code

## Documentation

### Getting started

Let's start off simple with a "Hello, World" program

```js
print("Hello, World")
```

Let's start defining some variables to make our code cleaner and life easier

```js
var message = "Hello, World!"

print(message)
```

Let's improve the code a bit

```js
const message = "Hello, World!"

print("$message <- what a cool message")
```

output:

```text
Hello, World! <- what a cool message
```

Let's look at what we have done here.

First we changed from `var` to `const` which means we can not modify the variable. This leads to better performance.

Instead of printing the value we referenced it in the message string using `$`. This allows us to better manipulate the output text and makes it a lot cleaner overall.

For more information look at the [docs](docs).

## Contributors

This project wouldn't have been possible without the help of these amazing people!

- [Thepigcat76](https://github.com/Thepigcat76) - **Project lead** and **lead dev**

- [TheHackerChampion](https://github.com/TheHackerChampion) - **Developer** and **Design team**

- [ReadyPlayerOne14](https://github.com/ReadyPlayerOne14) - **Contributor** and **Design team**
