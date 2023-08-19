# Fields

## Introduction

Fields are our way of allowing you to be more productive and making your code cleaner.

## Example

### Variables

> **Using normal code:**
>
> ```lua
> local const RED = (255, 0, 0)
> local const GREEN = (0, 255, 0)
> local const BLUE = (0, 0, 255)
> ```
>
> **Using fields:**
>
> To reduce boiler plate we introduce a field `local const` because every value shares these properties (local, constant)
>
> ```lua
> local const {
>    RED = (255, 0, 0)
>    GREEN = (0, 255, 0)
>    BLUE = (0, 0, 255)
> }
> ```

## EVERYTHING BELOW IS NOT UP TO DATE

## Required Fields

> While we try to be as dynamic as possible, some keywords are required to be fields
> For example `use, deps`
>
> ```rust
> use {
>   core.fmt, // optional commas
>   core.time,
>   gl.shader, 
> }
> 
> // deps can only be used in a .nxproj file
> deps {
>   nexusGL as gl, // dependcy alias
>   guiExtended as gui_ext, // all comas are optional
>   discord_api,
> }
> ```

### The `tag` field

> While `tags` can be used without fields, they are pretty useless without them.
> Tags are similar to `namespaces` in C# and `public classes` in Java.
> They can be used to change the appearence of a code block
>
> ```go
> tag Human // useless
>
> tag Animal {
>   func makeSound() {
>       print("Woof")   
>   }   
> }
>
> Animal.makeSound()
> ```
>
> When accesing this method from a different file you don't need to call it with the file name but can instead call the name of the tag. This is why tag names can't be identical.
