
# Acess modifier

## Directories

1. 'crit' / 'critical'
    - When using nexus files from a critical directory the interpreter will throw a warning unless the import allows usage of critical files

2. 'private' / 'unacess'
    - Nexus files in this directory cannot be accessed by the sub file

## Structs, enums, functions, vars

1. 'local'
    - Allows you to prevent other files from accessing the specified struct, enum, function or var

```text
local var meow = "Hello"

local enum Names {
    JAMES,
    LENA,
    ADDY
}

local struct ContactInfo {
    name: String,
    age: num
}

local fun sign_in name, age {
    var contact = ContactInfo(name, age)
}
```
