```rs
{
    let a = 1;
}
```

先变成一条链式的:> [{] -> [let] -> [a] -> [=] -> [1] -> [;] -> [}]

```
scope {
    other...,
    scope...
}
```

再判断它是一个赋值语句，生成一个
(statement variable a value 1)

```rs
Node<0> {
    type: scope
    range: []
    children： [
        <1>
    ]
}
Node<1> {
    type: statement,
    variable: string;
    value: string;
    range: []
}
```

```rs
match str {
    type.let (node) => {
        node
        next();
    }
    type.var:() => {
        next();
    }
    type.end:() => {
        next();
    }
}
```
