#![allow(warnings)]

use layout::gv;
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

