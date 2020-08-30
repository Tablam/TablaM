Is a pleasure for me ([Mario](https://github.com/mamcx)) & my friend ([Sebastian](https://github.com/hbro23)) to present our entry for this contest. 

I start programming in the 1990s with [FoxPro](https://en.wikipedia.org/wiki/FoxPro), a full-stack programming environment (at that time we don't use cool names like "*full-stack*") where was natural to use databases... and it was *VERY* productive. 

You can do *ALL* (forms, reports, databases, utilities, etc) with a single language & environment.

Microsoft, in a very weird move, kill it together with Visual Basic (shocking because was the MOST popular lang at the time) when it moves to .NET. So everyone jumps off the ship and the only remanent of the idea is today represented by Access and a few other tools, nearly all proprietary, stuck in the past or in what today is named "NoSql/BigData" that have a *higher* complexity.

Now, after using more than 12+ languages and more frameworks and libraries I can count, I wanna get back that experience: **A tool tailored for data**, where data is as important as code and the way to use it don't require ORMs or others tools that cause the well know [impedance mismatch](https://en.wikipedia.org/wiki/Impedance_matching).

This means that were most languages are focused on low-level details or engineering at large, **TablaM** is tailored with some small & big design decisions to make it enjoyable to write applications for e-commerce, finance, ERPs, and similars.

**The most distinctive feature of the language** is the use of the *relational model* as the base for its operation. This allows us to provide a set of *universal and consistent* [operations](/operators) that make  it easier to manipulate data. 

For example, where most languages have different methods to "*query*" items (sometimes called `find`, other times `get` or `search` or `filter` or ...), meaning you need to learn all different ways that change according to the context or kind of value you are using, in **TablaM** is just `?where`, and *ALL* values, even files, databases, sockets, values, numbers, text, etc... can be queried with the *same set of operators*. And you don't need to code them, because are universally available.

A small taste of the language as planned (what work today is in the [docs](https://tablam.org)):

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
- Easy to embed in another languages, similar to Lua.  **(tbd)**
- Multi-paradigm, but strongly based on the relational model. Also provide functional, array, imperative capabilities.
- Immutable values are the *default*, but allow mutable ones.
- Null safe.
- Provide decimal math as the default, instead of binary floating-point, making it better for business applications.
- Provide Algebraic Data Types  **(tbd)**.
- Provide SQL/LINQ-like experience across any relation. Like other languages say "*anything is an object*", in **TablaM**, "*anything is a relation*". 
- Built-in support for a variety of protocols, formats, and data transformation  **(tbd)**.
- No needs for ORMs. Talk directly to major RDBMS/SQL databases (PostgreSQL, MySql, SQL Server, SQLite) **(tbd)**.

We have prepare this material for this contest:

[Oficial Site](https://tablam.org)

[Tutorial](https://tablam.org/tutorial)

[Syntax](https://tablam.org/syntax)

[Small standard library](https://tablam.org/functions)

[Run on Relp.It](https://repl.it/@mamcx/RelpIt)

Video (in Spanish with subtitles in english, we are from Colombia!)