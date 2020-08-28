+++
title = "Tutorial"

+++

**!warning**: This tutorial is in progress, and the code is not yet functional

## Prerequisites

First, install or run the language in the browser, [as explained here](/install).

When you run the executable called "tabla" it enter in the "[repl](https://en.wikipedia.org/wiki/Read–eval–print_loop)" mode, in the terminal. You write the code and press "Enter" to execute.

You can also write the code in a text file with the extension ".tbm", and use a programming text editor like [Sublime](http://www.sublimetext.com) or [Visual Studio Code](https://code.visualstudio.com), then call `tablam name_file.tbm` to execute it.

## Introduction

Normally the tutorial of a programming language starts with the famous `"hello world"[^1].

Then it shows some small taste of syntax and later maybe it pretends you read the rest of the (potentially large) documentation and somehow, you "get it".

We, instead, will do something *different*. We build a *simple* yet functional program (a mini shopping cart) that shows what is the point of the language.

But first is necessary to talk about what *kind* of programming language **TablaM** is. Is based on a paradigm called "[Relational Model](https://en.wikipedia.org/wiki/Relational_model)". What is that? You will learn almost everything just staring, *intensely*, at this table:

| name         | price | qty  |
| ------------ | ----- | ---- |
| Hamburger    | 10    | 2    |
| Soda         | 5     | 4    |
| French fries | 7     | 2    |

<center>A <b>table</b>. Also a <b>"relation".</b></center>

You probably have guessed a lot of things just looking at this table. It looks like it talks about sales for a fast food store. You get an idea of what kind of products are available, at what prices and what quantities were given.

This is what makes the relational model its power. Is fairly "visual" and considers the data as "[first-class citizen](https://en.wikipedia.org/wiki/First-class_citizen)". But also, exist many other things that the table tell us about:

- It has a header that labels the columns (name, price, qty)
- It has columns, and their values are *homogeneous*
- It has rows, and their value (the row), represents a single entity of the relation

Now, to express this relation in TablaM, you need to write in the repl, or in a text editor, with a file called "sales.tbm":

```rust
let sales:= [
  name:Str, price:Dec, qty:Int;
  "Hamburger", 10, 2;
  "Soda", 5, 4;
  "French fries", 7, 2;
]
```



[^1]: And is `print("hello world")`, by the way.