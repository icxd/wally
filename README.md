# wally
wally is a statically-typed programming language inspired by Java, C, C++ and more. 

# Syntax Highlighting
To get any sort of syntax highlighting for wally inside Visual Studio Code, you have to clone the repository and place the 'wally' folder (inside the 'syntax' folder) into your 'C:/Users/whatever/.vscode/extensions/'

# Execute code
To execute your code, you have to have rust and cargo installed on your computer.
After that, you can run `cargo run filename.wly`, replacing the filename with the name of your program.

## TODO
- [x] Lexer
- [ ] Parser
  - [x] Variable declarations
  - [ ] Function declarations
  - [ ] Function calls
  - [ ] Valid maths parsing
- [ ] Validate AST
- [ ] Generate Assembly Code
- [ ] Validate Assembly
- [ ] Implement everything else
  - [ ] All datatypes
    - [ ] Strings
        - [ ] `string#length()` returns the length of the string
        - [ ] `string#charAt(index)` gets the character at the given index.
        - [ ] `string#indexOf(char)` gets the index of the given character.
        - [ ] `string#replace(string, string)` replaces all instances of the first string with the second string.
        - [ ] `string#split(string)` splits the string into an array of strings, using the given string as a delimiter.
        - [ ] `string#toUppercase()` converts the string to uppercase.
        - [ ] `string#toLowercase()` converts the string to lowercase.
        - [ ] `string#trim()` removes all whitespace from the beginning and end of the string.
    - [ ] Integers
      - [ ] 32-bit int
      - [ ] 64-bit int
      - [ ] 32-bit unsigned int
      - [ ] 64-bit unsigned int
    - [ ] Floats - 32-bit float
    - [ ] Doubles - 64-bit float
    - [ ] Shorts - 16-bit integer / float
    - [ ] Long - 128-bit integer / float
    - [ ] Bytes
    - [ ] Booleans
    - [ ] Chars
    - [ ] Voids - basically any type (not technically valid)
  - [ ] Arrays
    - An array is a list of values of the same type.
    - You can create an array of any type by writing 'array<type>'
    - [ ] `array#length()` The length of the array
    - [ ] `array#append(value)` Push a value to the end of the array
    - [ ] `array#remove(index)` Remove a value from the array at the given index
    - [ ] `array#insert(index, value)` Insert a value into the array at the given index
    - [ ] `array#get(index)` Get a value from the array at the given index
    - [ ] `array#clear()` Remove all values from the array
    - [ ] `array#sort()` Sort the array
    - [ ] `array#reverse()` Reverse the array
    - [ ] `array#copy()` Copy the array
    - [ ] `array#contains(value)` Check if the array contains the given value
    - [ ] `array#indexOf(value)` Get the index of the given value in the array
    - [ ] `array#toString()` Convert the array to a string
    - [ ] `array#forEach(lambda)` Loop through the array and call the lambda function for each value
  - [ ] Maps
    - A map is a list of key-value pairs.
    - You can create a map of any type by writing 'map<type, type>'
    - [ ] `map#length()` The length of the map
    - [ ] `map#keys()` An array of all the keys in the map
    - [ ] `map#values()` An array of all the values in the map
    - [ ] `map#append(key, value)` Add a key-value pair to the map
    - [ ] `map#remove(key)` Remove a key-value pair from the map
    - [ ] `map#clear()` Remove all key-value pairs from the map
    - [ ] `map#get(key)` Returns the value of the given key if it exists
    - [ ] `map#containsKey(key)` Check if the map contains the given key
    - [ ] `map#containsValue(value)` Check if the map contains the given value
    - [ ] `map#indexOfKey(key)` Get the index of the given key in the map
    - [ ] `map#indexOfValue(value)` Get the index of the given value in the map
    - [ ] `map#toString()` Convert the map to a string
  - [ ] Operators
    - If a type is not specified, it is assumed to be `void`, meaning that it does not have a return type, meaning that it cannot return anything.
    - You can also use a `?` as a type, meaing that it can return anything. This is not recommended, as it can cause errors and make the program harder to read.
    - [x] `+` - Add
    - [x] `-` - Subtract
    - [x] `*` - Multiply
    - [x] `/` - Divide
    - [x] `%` - Modulo
    - [x] `++` - Increment
    - [x] `--` - Decrement
    - [x] `+=` - Add by value
    - [x] `-=` - Subtract by value
    - [x] `*=` - Multiply by value
    - [x] `/=` - Divide by value
    - [x] `%=` - Mod by value
    - [x] `==` - Equals
    - [x] `!=` - Not equals
    - [x] `>` - Greater than
    - [x] `<` - Less than
    - [x] `>=` Greater than or equals
    - [x] `<=` - Less than or equals
    - [x] `&&` - Logical AND
    - [x] `||` - Logical OR
    - [x] `!` - Logical NOT
    - [x] `&` - Bitwise AND
    - [x] `|` - Bitwise OR
    - [x] `^` - Bitwise XOR
    - [x] `~` - Bitwise NOT
    - [x] `<<` - Bitwise left shift
    - [x] `>>` - Bitwise right shift
    - [ ] `>>>` - Bitwise unsigned right shift
    - [x] `&=` - Bitwise AND by value
    - [x] `|=` - Bitwise OR by value
    - [x] `^=` - Bitwise XOR by value
    - [x] `~=` - Bitwise NOT by value
    - [x] `<<=` - Bitwise left shift by value
    - [x] `>>=` - Bitwise right shift by value
    - [ ] `>>>=` - Bitwise unsigned right shift by value
  - [ ] Pointers
    - Pointers are a way to store the address of a variable. You can create a pointer to any type by writing `int*`, `string*`, `array<int>*`, etc.
    - You can dereference a pointer by writing `*pointerName`.
    - If you create a pointer to a variable, you will have to manually free the memory when you are done with it by writing `free(variableName);`.
  - [ ] References
    - A reference is used to reference another variable. You can create a reference to any type by writing `&variableName`.
    - To assign a variable to a pointer, you have to write `x: int* = &y;` instead of `x: int* = y;`.
    - If you don't want a referenced variable to reference another variable anymore, you can write `&variableName` and it will keep the value from the ex-referenced variable.
  - [ ] Objects
    - Objects are what every other type and class is based on.
    - You can create an object by writing 'object'.
    - [ ] `object#toString()` Convert the object to a string
    - [ ] `object#equals(object)` Check if the object is equal to another object

## Hello, World!
```
const { println } = import("io");

main: func<int> = (args?: array<string>) => {
    println("Hello, world!");
    return 0;
}
```