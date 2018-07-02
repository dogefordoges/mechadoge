# mechadoge

Esoteric programming language inspired by [dogescript](https://github.com/dogescript/dogescript/blob/master/LANGUAGE.md), completely written in Rust.

# Language Spec

## Assigning Variables
Define a variable like so:
```
very doge "much wow"
```

## Using Functions

Creating an anonymous function is simple. Use the `much` keyword to start the function, and the rest of the words in that line are considered the parameter name. The last value in the function is returned. And we end the scope of the function with the `wow` keyword.
```
much dogefun foo
   foo
wow
```

The above function simply returns `foo`.

Functions are *first class* so we can assign them and pass them as arguments to other functions

```
very dogefun much foo
   plz foo "doge"
wow

plz dogefun much bar
   plz bark bar
wow
```

The above example introduces some new keywords: `plz` and `bark`. `plz` is how we invoke a function call. `bark` prints whatever is passed to the terminal.

So what should be output to the screen is "doge". Since we pass a function to dogefun that barks whatever is passed to it. And we define dogefun to take a parameter `foo` and then invoke it with the argument "doge".

## Data structures

mechadoge uses the longboi data format to represent vectors and hashmaps.

Vectors are represented like this:
```
very foobar long "foo" "bar" boi
```

Here we assigned the `foobar` variable with a vector that contains two items, "foo" and "bar". Elements are separated with spaces. The end of the vector is denoted with boi.

Hashmaps are defined similarly, but with a few differences.
```
very foobar curly "foo" "bar" boi
```

Instead of `long` we use the keyword `curly` to start the hashmap. Key value pairs are denoted with two values, so "foo" is the key, and "bar" is the value. Once again we end the hashmap with boi.

## Comments

Anything after `shh` on a line is commented out
```
very doge "foo" ssh this is commented out
```

We can do multiline comments with `quiet` and `loud`.

```
quiet
This is a
multiline comment
loud
```

## Types
Mechadoge is dynamically typed but supports a few basic types Integers, Floats, Vectors, Characters, Strings, HashMaps, and Functions.

## Modules

To define a module (namespace) in mechadoge use the `such` keyword, and end it with `wow`.

```
such doge
   very hello much foo
      plz bark 'hello'
   wow
wow
```

We can import modules which will import all the methods, or use an alias:

```
so doge

hello
```

Or use an alias:
```
so doge as d

d.hello
```

Or to import specific methods
```
so doge long hello boi

hello
```

## File Extension
mechadoge source code files are defined with a `.mdg`, and longboi data files are `.lboi`.
  