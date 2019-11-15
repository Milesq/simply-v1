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
