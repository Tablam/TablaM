+++
title = "Learn the TablaM syntax is few minutes"

+++

**Note**: Things marked with (**TBD**) mean that feature is not yet functional.

**TablaM** is a language that combine relational, functional & imperative paradigms. Is expression based, meaning all code return values. 

The syntax is made to be learned quickly, and be as consistent as possible.

### Comments

```sql
-- Comments start with 2 'minus' symbol
```

## Declarations

By *default*, the values are immutable except if explicitly marked as mutable. To declare a value you write:

```swift
let x := 1 -- inmmutable
var x := 1 -- mutable
```

Note how to assign a value is used the operator `:=`. This is different to the equal operator `=` that is used to compare for equality.

## Literals

Literals are values that are typed directly in the source code, and are the most *primitive values* of a programming language:

### Numbers

```sql
1 --64 bit Integers
1.0 --64 bit Decimals (base 10)
1.0d --64 bit Decimals (with explicit suffix)
1.0f --64 bit floats (base 2, requiered explicit suffix)
```

Note how the `Decimal` is the default, instead of the `float`. This mean **TablaM** is tailored to do more exact arithmetic for business/financial purposes, instead of default for engineering like most languages.

With numbers it is possible to do some

### math

```rust
1 + 1 -- = 2
2.0 - 1.0 -- = 1.0d
2.0f * 10.0f -- = 20.0f
10 / 2 -- = 5.0
1 + 2.0 -- Error: Type mismatch Int <> Dec
```

**TablaM** doesn't do invisible conversions of values. Is necessary to explicitly cast values to mix them. (**TBD**).

### Booleans

```sql
true
false
```

With boolean values we can do *boolean expressions*, so we can

### Compare values (**TBD**):

```python
1 == 1 -- = true
1 != 1 -- = false

1 > 2 -- = false
1 >= 2 -- = false

1 < 2 -- = true
1 <= 2 -- = true

true and false -- = true
not true -- = false
```

**TablaM** have the concept of a "*total order*".It means all values can be compared in relation to the others. This is required to things like sorting to work. The total order is defined as:

```rust
Bit < Bool < Int < Float < Dec < Time < Date < DateTime <
Char < Str < Vec < Tree < Map < Any
```

You don't need to memorize this. Only know that this feature exist.

### Strings

All strings are valid [UTF8](https://en.wikipedia.org/wiki/UTF-8).

```sql
"hello" -- Strings are encoded with quotes
'hello' -- both '' or "".

"hello it's you" --so you can embed easily the opposite quote

"""
A long text
""" --- and triple quotes allow to enter longer text

"🍎" -- unicode can be used!
```

### Dates

Dates can be entered and validated at compile time if the string is prefixed & formatted as ISO date:

```sql
dt"2020-02-02T08:00:00" -- Full date time 
d"2020-02-02" -- Just date time 
t"08:00:00" -- Just time 
```

## Types

**TablaM** use a *static type system*, meaning all values are assigned a *type* and is not possible to use values where it not work (**TBD**, *for now the types are checked at runtime*).

Types are described in *TitleCase* like this:

```rust
Int
Dec
Float
Str
Date
DateTime
Time
Vec[it:Int]
Map[name:Str, age:Int]
```

Note how some values have a list of types inside []. This is called the schema (or header) of the relation. ALL values have a schema:

```rust
Int = [it:Int]
Str = [it:Str]
Map[name:Str, age:Int] = [name:Str, age:Int]
```

This schema allow the *relational operators* to work and treat the values as if where tables in *SQL*.

## Relations

**The main feature of the language is the view of all values as *relations***. A relation is *anything* that can be described like "have a list of names: Types", "It has rows that match the header", and "it has columns of homogeneous values".

### Scalars

A scalar is a relation of a unique value, a single column, a single row. 

Examples of scalars are:

```rust
1
true
"hello"
```

Looking at them as relation, are identical to:

```rust
1 = [Int; 1]
true = [Bool; true]
"hello" = [Str; "hello"]
```

This means that the scalar is really a vector in disguise.

### Vectors

A vector is a collection of contiguous scalars that are of the same type. Can be considered similar to a "*column*" in a table.

Examples of vectors are:

```rust
[1; 2; 3] -- A vector of Ints
[1] -- A vector of Ints

[Int; 1; 2; 3] -- A vector of Ints, with a explicit schema with just the type. The name by default in vectos is "it"

[age: Int; 1; 2; 3] -- A vector with a explicit schema with name "age" and type Int

[city:Str, country:Str; 
  "miami", "USA"; 
  "bogota", "Colombia"] -- A 2d vector that is like a table
```

Note how `,` is used to separate values (aka: *cells*) and `;` is to separate rows.

The vector is *ordered exactly as was entered* (or loaded form any source), and **allow duplicated rows**.

### Tree (TBD)

A Tree is a collection of rows. The rows are stored internally in a [B-Tree](https://en.wikipedia.org/wiki/B-tree), meaning the rows are *ordered according to a key* that must be explicitly defined using the *total order* described before.

The name Tree can convey the idea of a hierarchy of values in most languages. But remember that this is a relational language and see all things as relations.

Examples of trees are:

```matlab
Tree[|
  pk id:Int, name:Str;
  1, "hello"; -- row0
  2, "world"  -- row1
|]
```

Note the use of `[||]` to enclose the data, that it must be preceded by the type `Tree`, and the keyword `pk` is used to define the key for comparison and fast search by that `pk`. The presence of a key also mean that  **duplicated rows are replaced with the last pk**.

## Relational operators

The *relational operators* are the second most distinctive feature of the language. This are *intrinsically* part of the relational model. Where exist a relation, you can be certain you can apply *ALL* the relational operators. Them are [described in their own page](/operators).

All relational operators must start with the operator query `?`.

Examples of relational operators are:

```sql
[1; 2; 3] ?where #0 = 2 -- Filter only values exactly = 2
open("file.csv") ?sort ?limit 3 -- Open a file, sort it and return the first 3 rows

--You can use them everywhere
for i in [1; 2; 3] ?skip 2 -- skip the first 2 rows
```

## Functions

The majority of the functionality is provided with functions. A function is a piece of code that perform varied task to fulfils a need. 

To use a function:

```rust
print(1) -- print the value to stdout (aka: Terminal)
sum([1;2;3]) --sum all the values
```

The available functions are [described in their own page](/functions).

## Control flow

In **TablaM** is more idiomatic to use the relational operator or use a more functional style, but is possible to use imperative control flow:

### if

```rust
let value := 
if true do
   1
else 
   2
end
```

Note how you can return the last value in each if branch.

However, this is not possible for looping constructs, them return the "pass"  value. Is similar to the void in other languages.

### while

```rust
while true do
   println("this repeat forever")
end
-- return pass
```

### for

```rust
for i in 1..10 do --this count from 1 to 9
   println(i)
end
-- return pass
```

