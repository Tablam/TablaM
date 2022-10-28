+++
title = "Relational Operators"

+++

All relational operators need to be expressed as:

```tablam
a_relation ?a_operator parameters...
```

The character `?` is called "*query*".

### ?select and ?deselect:

The `?select` operator (aka: projection or `SELECT * FROM` in sql), allow filtering *the columns* in a relation. Using the character `#` to indicate that is a name (like `#price`) or a number (like `#0`) of a column:

```tablam
let products := sales ?select #name
print(products)
> 
```

You can also use `?deselect` to say which columns NOT pick:

```tablam
let products := sales ?deselect #name
print(products) //it show the all the columns, except #name
```

### ?where:

The `?where` operator (aka: selection or `where...` in sql), allow filtering *the rows* in a relation. It needs a "*boolean expression*", ie: expression that compares values, columns or returns true/false.

```tablam
let soda := sales ?where #name == "Soda"
let soda := sales ?where #0 == "Soda"

let cheaper := sales ?where #price < 5.0

```

### ?limit:

The `?limit N` operator return up to N rows from the query. If the value supplied is bigger than the total of rows, then it will return all rows.

```tablam
let products := sales ?limit 1
```

### ?skip:

The `?skip N` operator skip N rows from the query and return the rest. If the value supplied is smaller than the total of rows, then it will return nothing.

```tablam
let products := sales ?skip 1
```

### ?distinct:

The `?distinct` operator avoid duplicated values.

```tablam
let products := sales ?distinct
```

