# mechadoge

Esoteric programming language inspired by [dogescript](https://github.com/dogescript/dogescript/blob/master/LANGUAGE.md), completely written in Rust.

# Language Spec

## Assigning Variables
Define a variable like so:
```
very doge "much wow"
```

## Types
Mechadoge is dynamically typed but supports a few basic types Integers, Floats, Strings, and Booleans. Mechadoge also supports inline array representations of these values, which is discussed in the data structures section.

The basic types work as expected, however, boolean expressions have a special representation.

`1=1` is the default representation of `true`, and `1=2` is `false`. Any boolean will be output in this form, however, booleans can also be represented with different numbers. Any equivalent numbers surrounding the `=` are represented as true, Ex: `40=40` is true. Whereas any values that are not equivalent are false, Ex: `40=41` is false.

## Functions

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

### Returning data from functions
Mechadoge uses an implicit return statement, which means the last thing in the function body is what is returned. When passing the returned value to another function or keyword, position the returning function before the function expecting the value.

```
shh returns a string
very funz much
   "this is so much fun!!!"
wow

plz funz shh returns the string
very fun_var shh assigns the string to fun_var
```

This may seem weird for simple expressions, but makes it very easy to chain function calls together. Where the returned value will always be used as the last parameter for the next function.

```
plz fun
plz bar 0
plz foobar 42 "doges"
very mecha
```

## Control Flow

Mechadoge currently only supports simple if statements using the `rly` keyword. It works as a conditional function call. It expects a boolean value, and a parameter-less function. Here is an example:

```
1=1
rly much
   plz bark "Hello, world!"
wow
```

This will print "Hello, World!" to the terminal. Because `1=1` is true. Whereas the next example will output nothing because the `much` expression is never called.

```
1=2
rly much
   plz bark "Hello, world!"
wow
```

You can also use boolean expressions:
```
plz smaller 4 5
rly much
   plz bark "Hello, there!"
wow
```

## Data structures

mechadoge uses the longboi data format. The only data structure available is vectors.

Vectors are represented like this:
```
very foobar long "foo" "bar" boi
```

Here we assigned the `foobar` variable with a vector that contains two items, "foo" and "bar". Elements are separated with spaces. The end of the vector is denoted with boi.

### Hash Maps

If a long boi array has an even number of elements it can be converted into a hashmap with the `curly` function. 

```
plz curly long "foo" "bar" boi
very foobar
```

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

## File Extension
mechadoge source code files are defined with a `.mdg`, and longboi data files are `.lboi`.