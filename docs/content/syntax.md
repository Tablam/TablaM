+++
title = "Learn the TablaM syntax is few minutes"

+++

**Note**: Things marked with (**TBD**) mean that feature is not yet functional.

**TablaM** is a language that combine relational, functional & imperative paradigms. Is expression based, meaning all code return values. 

The syntax is made to be learned quickly, and be as consistent as possible.

### Comments

```tablam
-- Comments start with 2 'minus' symbol
--- 
--- Multiline? just repeat it!
---
```

## Declarations

By *default*, the values are immutable except if explicitly marked as mutable **(TBD)**. To declare a value you write:

```tablam
let x := 1 -- immutable aka: final (java), const (js), it cannot be updated
var x := 1 -- mutable
```

Note how to ***assign a value*** is used the operator `:=`. This is different to  `=` that is used to ***compare for equality***.

## Literals

Literals are values that are typed directly in the source code, and are the most *primitive values* of a programming language:

### Numbers

```tablam
1 --64 bit Integers
1.0 --64 bit Decimals (base 10)
1.0d --64 bit Decimals (with explicit suffix)
1.0f --64 bit floats (base 2, requiered explicit suffix)
```

Note how the `Decimal` is the **default floating point number**, instead of the `float`, that is more common. 

This mean **TablaM** is tailored to do [more exact arithmetic](https://stackoverflow.com/questions/618535/difference-between-decimal-float-and-double-in-net) for business/financial purposes, instead of default for engineering like most languages.

With numbers it is possible to do some arithmetic and assignment operators 

### Math

```tablam
1 + 1 -- = 2
2.0 - 1.0 -- = 1.0d
2.0f * 10.0f -- = 20.0f
10 / 2 -- = 5.0
1 + 2.0 -- Error: Type mismatch Int <> Dec
let num := 1
num += 1 -- arithmetic and assignment operators (TBD)
```

**TablaM** doesn't do invisible conversions of values. Is necessary to explicitly cast values to mix them. (**TBD**).

Similar to array languages/libraries like APL/kdb+/NumPy, the operators know how to work with *scalars* & *collections* like:

```tablam
1 + 1 -- Scalar + Scalar
1 + [1; 2; 3] -- Scalar + Vector
[1; 2] + [3; 4] -- Vector + Vector, same rank
[1] + [3; 4] -- ERROR, different rank 1 <> 2
```

### Booleans & Bits

```tablam
true
false
1b        -- Bit 1
0b        -- Bit 0
010_101_01b --Bit Array, can use _ for clarity
```

With Boolean values we can do *Boolean expressions*, so we can

### Compare values (**TBD**):

```tablam
1 = 1 -- = true
1 <> 1 -- = false

1 > 2 -- = false
1 >= 2 -- = false

1 < 2 -- = true
1 <= 2 -- = true

true and false -- = true
not true -- = false
```

and with bit/bitarrays do bit manipulation:

### Bit operations (**TBD**):

```tablam
let a = 0b0
let b = 0b_1b

a.and(b)
a.or(b)
a.xor(b)
a.not(b)
a.shift_left(b)
a.shift_right(b)
a.shift_right_zero(b)
```

**TablaM** has the concept of a "*total order*". It means all values can be compared in relation to the others. This is required for things like sorting to work. The total order is defined as:

```tablam
Bit < Bool < Int < Float < Dec < Time < Date < DateTime <
Char < Str < Vec < Tree < Map < FFI Object < Any
```

You don't need to memorize this. Only to know that this feature exist.

### Strings

All strings are encoded in valid [UTF8](https://en.wikipedia.org/wiki/UTF-8).

```tablam
"hello" -- Strings are enclosed with quotes
'hello' -- both '' or "".

"hello it's you" --so you can embed easily the opposite quote

"""
A long text
""" --- and triple quotes allow to enter longer text

"🍎" -- unicode can be used!
```

### Dates (**TBD**)

Dates can be entered and validated at compile time if the string is prefixed & formatted as ISO date:

```tablam
dt"2020-02-02T08:00:00" -- Full date time 
d"2020-02-02" -- Just date time 
t"08:00:00" -- Just time 
```

## Types

**TablaM** use a *static type system*, meaning all values are assigned a *type* and is not possible to use values where it doesn’t work (**TBD**, *for **now** the types are checked at runtime*).

Types are described in *TitleCase* like this:

```tablam
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

Note how some values have a list of types inside `[]`. This is called *the schema* (or header) of the relation. *ALL values have a schema*:

```tablam
Int = [it:Int]
Str = [it:Str]
Map[name:Str, age:Int] = [name:Str, age:Int]
```

This schema allows the *relational operators* to work and treat the values as they were tables in *SQL*.

## Relations

**The main feature of the language is the view of all values as *relations***. A relation is *anything* that can be described like "have a list of `names`: `Types`", "It has rows that match the header", and "it has columns of homogeneous values".

### Scalars

A scalar is a relation of a *unique value*, a single column, a single row. 

Examples of scalars are:

```tablam
1
true
"hello"
```

Looking at them as relation, are identical to:

```tablam
1 = [Int; 1]
true = [Bool; true]
"hello" = [Str; "hello"]
```

This means that the scalar is really a vector in disguise. This also mean that anything can be considered a scalar if is manipulated *as a whole* (this is the default in most languages).

### Vectors

A vector is a collection of contiguous scalars that are of the *same type*. Can be considered similar to a "*column*" in a table.

Examples of vectors are:

```tablam
[1; 2; 3] -- A vector of Ints
1 -- A vector of Ints

 -- A vector of Ints, with a explicit schema with just the type. The name by default in vectos is "it"
[Int; 1; 2; 3]

-- A vector with a explicit schema with name "age" and type Int
[age: Int; 1; 2; 3] 

-- A 2d vector. Normal people call it a table
[city:Str, country:Str; 
  "miami", "USA"; 
  "bogota", "Colombia"] 
```

Note how `,` is used to separate values (aka: *cells*) and `;` is to separate rows.

The vector is *ordered exactly as was entered* (or loaded form any source), and **allow duplicated rows**.

### Tree (TBD)

A Tree is a collection of rows. The rows are stored internally in a [B-Tree](https://en.wikipedia.org/wiki/B-tree), meaning the rows are *ordered according to a key* that must be explicitly defined using the *total order* described before.

The name Tree can convey the idea of a hierarchy of values in most languages. But remember that this is a relational language and see all things as relations, but this still allow to express a hierarchy (is a Tree), just that the keys/values are like relations.

Examples of trees are:

```tablam
Tree[|
  pk id:Int, name:Str;
  1, "hello"; -- row0
  2, "world"  -- row1
|]
```

Note the use of `[||]` to enclose the data, that it must be preceded by the type `Tree`, and the keyword `pk` is used to define the key for comparison and fast search by that `pk`. The presence of a key also mean that  **duplicated rows are replaced with the last row of that pk**.

## Enums (TBD)

To declare enums:

```tablam
enum Status do
    case Active
    case Inactive


enum Option[T] do
  case Some(T)
  case None

enum Value do
    case Str(String)
    case Num(Int)
    case Dec(Decimal)

enum Value2:Value do
    case Bool(Bool)

enum ValueNums: Value
    .Num, .Dec
```

To pattern match on enums

```tablam
match status do
case Active do
    "active"
case Inactive do
    "Inactive"
end

match optional do
case Some(x) do
    x
case None do
   abort("No value")
end
```

## Relational operators

The *relational operators* are the second most distinctive feature of the language. Them are *intrinsically* part of the relational model. Where exist a relation, you can be sure you can apply *ALL* the relational operators. Them are **[described in their own page](/operators)**.

All relational operators must start with the operator *query* `?`.

Examples of relational operators are:

```tablam
-- Filter only values exactly = 2
[1; 2; 3] ?where #0 = 2 

-- Open a file, sort it and return the first 3 rows
open("file.csv") ?sort ?limit 3 

--You can use them everywhere
for i in [1; 2; 3] ?skip 2 do-- skip the first 2 rows
end
```

## Functions

Most of the functionality of the language is provided with functions. A function is a piece of code that perform varied task to fulfils a need. 

To declare a function:

```tablam
fun sum(a:Int, b:Int) = Int do
   a + b
end

fun total_invoice(inv:[|price:Money, qty: Money ...|]) = $inv[| total: Money |] do
   inv?select inv.price * inv.qty as total
end
```

To use a function:

```tablam
-- print the value to stdout (aka: Terminal)
print(1) 

-- sum all the values
sum([1;2;3])

-- sum all the values and then print, in pipeline notation
[1;2;3] | sum | print
```

The available functions are **[described in their own page](/functions).**

## Control flow

In **TablaM** is more idiomatic to use the relational operators or use a more functional style, but is possible to use imperative control flow:

### if

```tablam
let value := 
if true do
   1
else 
   2
end
```

Note how you can return the last value in each if branch.

However, this is not possible for looping constructs, these return the "pass"  value. Is similar to the void in other languages.

### while

```tablam
while true do
   println("this repeat forever")
end
-- return pass
```

### for

```tablam
for i in 1..10 do --this count from 1 to 9
   println(i)
end
-- return pass
```

## Types (tbd)

```tablam
type Invoice do
    price:Decimal
    qty:Decimal
    total:Decimal
end

impl Invoice 

fun save(self, db:Db) do
end

end

type i32 as InvoiceId  -- alias
type InvoiceId is i32  -- newtype
```

## Traits (tbd)

```tablam
trait Display do
    fun display(self)=String

impl Display for Invoice

fun display(self)=String do

end

end
```

## Modules (tbd)
```tablam
mod Invoices

end

import Invoices
```
