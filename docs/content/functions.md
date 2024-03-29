+++
title = "Functions"

+++

**TablaM** provides a small selection of functions available:

## Math

Operations like "`+,-,*,/`" are in fact functions internally. With them is also available:

### Sum

The `sum` function operate on  relations of numbers and return the total:

```tablam
fun sum(rel:[Number]) = Number
```

```tablam
sum(1.0f) -- = 1.0f
sum([1; 2; 3]) -- = 6
```

### Avg

The `avg` function operate on  relations of numbers and return the average of the values:

```tablam
fun avg(rel:[Number]) = Number
```

```tablam
avg(1.0f) -- = 1.0f
avg([1; 2; 3]) -- = 2
```

## Logic

### min & max

The `min`& `max` functions accept a relation and return the min or max value, accordingly.

Note that these are *logical function*s, not math. So it operates on *ANY* value following the total order of types.

```tablam
fun min(rel:Rel) = Rel
fun max(rel:Rel) = Rel
```

```tablam
min([1; 2]) -- = 1
max([1; 2]) -- = 2

min([true, 1; 2]) -- = true
max([1; 2; "Hello"]) -- = "Hello"
```

## IO

To operate with IO (files, stdin, stdout, sockets, etc):

### print & println

The `print`& `println` functions accept a list of values and convert them to `String` and output the result to the stdout, which is commonly the terminal.

```tablam
fun print(values: Any...)
fun println(values: Any...)
```

```tablam
print("hello")
print("world")

println("hello")
println("world")
```

### open

The `open` function load a `.csv` file, *scan the header* and return it as a relation. Note that this functionality is temporal. Later will be correctly used to open any kind of file.

```tablam
fun open(path: Path) = Rel
```

```tablam
-- With a csv file like
-- id,ref,name
-- 1,24236-097,Noodles
let products := open("products.csv")

```

### read_to_string

The `read_to_string` function take a `File` and load the contents as a `String`.

```tablam
fun read_to_string(file: File) = String
```

```tablam
let products := open("products.csv")
let txt := read_to_string(products)
```

### save

The `save` function turn the relation in a `String`, then save the results to the disk. Note that this functionally not yet work correctly.

```tablam
fun save(rel:Rel, path: Path)
```

```tablam
let products := [
  id:Int, ref:Str, name:Str; 
  1,"24236-097","Noodles"]

save(products, "products.csv")
```