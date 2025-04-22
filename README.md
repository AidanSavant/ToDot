# ToDot
An application in rust that converts k-ary trees into DOT programs that can be rendered. 

# Example
main.rs:
```rust
use to_dot_derive::ToDot;

#[derive(ToDot)]
struct Tree<T>
where
    T: ToString,
{
    #[value]
    value: T,

    #[children]
    children: Vec<Tree<T>>,
}

impl<T> Tree<T> 
where T: ToString {
    fn new(value: T, children: Vec<Tree<T>>) -> Self {
        Self { value, children }
    }
}

fn main() {
    let tree = Tree::new(
        '+',
        vec![
            Tree::new(
                '1',
                vec![
                    Tree::new(
                        '5',
                        vec![
                            Tree::new('x', vec![]),
                            Tree::new('y', vec![])
                        ]
                    )
                ]
            ),
            
            Tree::new('2', vec![]),
            Tree::new('3', vec![]),
        ]
    );

    println!("{}", tree.to_dot());
}
```

output:
```
digraph G {
        0 [label="+"]
        0 -> 1
        1 [label="1"]
        1 -> 2
        2 [label="5"]
        2 -> 3
        3 [label="x"]
        2 -> 4
        4 [label="y"]
        0 -> 5
        5 [label="2"]
        0 -> 6
        6 [label="3"]
}
```

result:
![rendered_dot](/imgs/dot.png)


