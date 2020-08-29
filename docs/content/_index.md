+++
title = "Home"
+++
# TablaM: A relational language #

**IN EXPERIMENTAL STAGE. NOT USE FOR REAL WORK**

**TablaM** is an *in-progress* programming language to provide a more ergonomic experience for building **data-oriented** applications.

This means that were most languages are focused on low-level details or engineering at large, **TablaM** is tailored with some small & big design decisions to make it enjoyable to write applications for e-commerce, finance, ERPs, and similars.

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
for p in cross products, qty ?limit 10 do
	print(p.products.price * p.qty)
end

```

So, what *kind* of language is **TablaM**?:

- Multi-paradigm, but strongly based on the relational model. Also provide functional, imperative capabilities.

- Immutable values are the *default*, but allow mutable ones.

- Provide SQL/LINQ-like experience across any relation. Like other langs say "*anything is an object*", in **TablaM**, "*anything is a relation*". 

  

### Code

[https://github.com/Tablam/TablaM/](https://github.com/Tablam/TablaM/)