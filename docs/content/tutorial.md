+++
title = "Tutorial"

+++

**!warning**: This tutorial is in progress, and the code is not yet functional

## Prerequisites

First, install or run the language in the browser, [as explained here](/install).

When you run the executable called "tablam" it enter in the "[repl](https://en.wikipedia.org/wiki/Read–eval–print_loop)" mode, in the terminal. You write the code and press "Enter" to execute.

You can also write the code in a text file with the extension ".tbm", and use a programming text editor like [Sublime](http://www.sublimetext.com) or [Visual Studio Code](https://code.visualstudio.com), then call `tablam name_file.tbm` to execute it.

## Introduction

Normally the tutorial of a programming language starts with the famous `"hello world"`[^1].

Then it shows some small taste of syntax and later maybe it pretends you read the rest of the (potentially large) documentation and somehow, you "get it".

We, instead, will do something *different*. We build a *simple* yet functional program (a mini shopping cart) that shows what is the point of the language.

But first is necessary to talk about what *kind* of programming language **TablaM** is. Is based on a paradigm called "[Relational Model](https://en.wikipedia.org/wiki/Relational_model)". What is that? You will learn almost everything just staring, *intensely*, at this table:

| name         | price | qty  |
| ------------ | ----- | ---- |
| Hamburger    | 10.2  | 2    |
| Soda         | 3.0   | 4    |
| French fries | 7.0   | 2    |

<center>A <b>table</b>. Also a <b>"relation".</b></center>

You probably have guessed a lot of things just looking at this table. It looks like it talks about sales for a fast food store. You get an idea of what kind of products are available, at what prices and what quantities were given.

This is what makes the relational model its power. Is fairly "visual" and considers the data as "[first-class citizen](https://en.wikipedia.org/wiki/First-class_citizen)". But also, exist many other things that the relation tell us about:

- It has a header that labels the columns (name, price, qty)
- It has columns, and their values are *homogeneous*
- It has rows, and their value (the row), represents a single entity of the relation

## Write the first program

Now, to express this relation in **TablaM**, you need to write in the *repl*, or a text editor, with a file called `sales.tbm`:

```rust
let sales:= [
  name:Str, price:Dec, qty:Int;
  "Hamburger", 10.2, 2;
  "Soda", 3.0, 4;
  "French fries", 7.0, 2;
]
```

Let's explain what all that text means:

- `let sales` create an immutable (read-only) binding named "sales"
- `:=` is the assignment operator. It put in the *left* what is on the *right* of it.
- Enclosed in `[]` is the relation, stored as a *vector*. Vectors are one of the ways to store data in computer memory. If you come from another language it could look strange to see that the vector allows rows and columns, instead of *only* flat values like `[1;2;3]`. But remember that this is a relational language!
- The first line declares the header or schema of the relation, with pairs of *names* & *types*. Types are, among other things, ways to define what kind of value the data/column *is*.

| name  | type                                                         | type usage                                                   |
| ----- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| name  | [Str](https://en.wikipedia.org/wiki/UTF-8)                   | For text, data in UTF8 format                                |
| price | [Dec](https://en.wikipedia.org/wiki/Decimal_data_type)       | For numbers, as 64 bit decimals, like money                  |
| qty   | [Int](https://en.wikipedia.org/wiki/Integer_%28computer_science%29) | For numbers, as 64 bit integers, like quantities, positions, counts, etc |

- The next lines are the "rows". It must match the type of their column.

 Now with this data, we can do a lot of stuff, thanks to:

## The relational operators

  We can use "*queries*" to manipulate the data stored in relations. This "*queries*" are called [***relational operators***](/operators) because they express different operations on relations. 

The character `?` is called "*query*".

We will only worry about this 2:

### ?select

The `?select` operator (aka: projection or `SELECT * FROM` in sql), allow filtering *the columns* in a relation. Using the character `#` to indicate that is a name (like `#price`) or a number (like `#0`) of a column:

```rust
let products := sales ?select #name
print(products)
> 
```

### ?where

The `?where` operator (aka: selection or `where...` in sql), allow filtering *the rows* in a relation. It needs a "*boolean expression*", ie: expression that compares values, columns or returns true/false.

```rust
let soda := sales ?where #name == "Soda"
let soda := sales ?where #0 == "Soda"

let cheaper := sales ?where #price < 5.0

```

## Some math

Now we can start to do more stuff. We can know how many money we get for this sale:

```rust
let profit_items := sales?select #price * #qty
print(profit_items)
let profit_total := sum(profit_items)
print(profit_total)
```

And which product give use the biggest profit:

```rust
let most := sales?select #price * #qty
let most := max(most)
print(most)
```

Note how each operation work in relations and return relations.

Single values like `1` or `"Soda"` are also relations. Know too as "[scalars](https://en.wikipedia.org/wiki/Variable_(computer_science))". TablaM considers it relations of 1 column, 1 row, and 1 cell.

This mean that this is possible:

```rust
let price := 1.0 ?select #0
```

Now, we can continue with the program and make it more useful:

```rust
	
```



[^1]: And is `print("hello world")`, by the way.