+++
title = "Home"
+++
# TablaM: A relational language #

**IN EXPERIMENTAL STAGE. NOT USE FOR REAL WORK**

**TablaM** is an *in-progress* programming language to provide a more ergonomic experience for building **data-oriented** applications.

This means that were most languages are focused on low-level details or engineering at large, **TablaM** is tailored with some small & big design decisions to make it enjoyable to write applications for e-commerce, finance, ERPs, and similars.

**The most distinctive feature of the language** is the use of the *relational model* as base for its operation. This allow to provide a set of *universal and consistent* [operations](/operators) that make easier to manipulate data. 

For example, where most languages have different methods to "*query*" items (sometimes called `find`, other times `get` or `search` or `filter` or ...), meaning you need to learn all different ways that change according to the context or kind of value you are using, in **TablaM** is just `?where`, and *ALL* values, even files, databases, sockets, values, numbers, text, etc... can be queried with the *same set of operators*. And you don't need to code them, because are universally available.

A small taste of the language:

```rust
-- A column, aka: Vectors...
let qty := [10.5, 4.0, 3.0] 
-- Like APL/kdb+ operations apply to more than scalars
prices * 16.0 

-- The ? (query) operator allow to do SQL-like queries to anything
let doubled := qty ?select #0 * 2.0 

let products := open("products.csv")
-- like files!
for p in products ?where #price > 0.0 do
    print(p)
end
--so, we can do joins between anything:
for p in cross(products, qty) ?limit 10 do
	print(p.products.price * p.qty)
end

```

So, what *kind* of language is **TablaM**?

*Note*: Items marked **(tbd)** are in the roadmap...

- Build on the high-performance [Rust language](https://www.reddit.com/r/rust/).

- Available to major 64-bit platforms (Linux, Windows, MacOS, iOS **(tbd)**, Android  **(tbd)**).

- Multi-paradigm, but strongly based on the relational model. Also provide functional, imperative capabilities.

- Immutable values are the *default*, but allow mutable ones.

- Null safe.

- Provide decimal math as the default, instead of binary floating-point, making it better for business applications.

- Provide Algebraic Data Types  **(tbd)**.

- Provide SQL/LINQ-like experience across any relation. Like other languages say "*anything is an object*", in **TablaM**, "*anything is a relation*". 

- Built-in support for a variety of protocols, formats and data transformation  **(tbd)**.

- No needs for ORMs. Talk directly to major RDBMS/SQL databases (PostgreSQL, MySql, SQL Server, SQLite) **(tbd)**.

  

### Code

[https://github.com/Tablam/TablaM/](https://github.com/Tablam/TablaM/)