# simply-v1

## This project has been deprecaed
Simply, useful language

Simply was planned as simply language dedicated for everyday problems / write interactive configurations and useful scripts.

You can see syntax example in `examples/mail.sl`. As you see it is really easy.


## What next?

I'll create next version of Simply. The whole interpreter may be rewrited, but syntax will be almost the same.

Simply will have two different kind of syntax (on the model of Sass and Scss) called `Airgel` (Python like syntax) and `Cobalt` ()

Old version:

```
func show x {
    print("x is equal", x)
    return x + 1
}

let x = 3
let y = @{
    q = x
    z = 0
}

show(y.q)

if x >= 2 {
    print("x is greater than two")
}

print("Hello, World")
```

(The following syntax may be changed)

New version (Airgel syntax):

```
func show x
    print "x is equal" x
    x + 1

let x = 3
let y =
    q = x
    z = 0

show y.q

if x >= 2
    print("x is greater than two")

print "Hello, World"
```

New version (Cobalt syntax):

```
func show x {
    print("x is equal", x)
    return x + 1
}

let x = 3
let y = object {
    q = x
    z = 0
}

show(y.q)

if (x >= 2) {
    print("x is greater than two")
}

print("Hello, World")
```

## Keywords explain

### Simply doesn't any keywords

`let`, `func`, `if`, `for` are just low level functions saved in Simply Core.

assignment mark (`=`) is just a _special value_

### Why?

It makes easier develop Simply Core, low level functions (extending Simply by other langueages), meta programming

## My vision

In the nexts versions Simply would have as ease as possible, and as powerful as possible std libraries for:
-   image/audio/video processing
-   control hardware options
-   making window app's
-   network connections
-   many many others

That's my vision of the perfect helper language
