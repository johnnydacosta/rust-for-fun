#![allow(unused_variables)]
#![allow(dead_code)]

use std::rc::Rc;

fn main() {
    //eg_immutable_reference();
    //eg_shadowing();
    // eg_traits();

    {
        let res: Result<String, String> = Ok(String::from("Hello, world!"));
        let res_or_default = match res {
            Ok(v) => {
                println!("v: {}", v);
                v
            }
            Err(e) => {
                println!("e: {}", e);
                String::from("default")
            }
        };
        println!("res_or_default: {}", res_or_default);
    }

    let res: Result<i32, String> = {
        let mut x = 5;
        let y = x; // y is a copy of x
        println!("x: {}, y: {}", x, y);
        x = 5;
        let y = &mut x;
        println!("y: {}", y);
        x = 4;
        println!("x: {}", x);
        Ok(x)
    };
    println!("res: {}", res.unwrap_or(3));

    {
        struct Node<T> {
            value: T,
            parent: Option<Rc<Node<T>>>,
        }

        let root = Rc::new (Node {
            value: "root",
            parent: None,
        });

        let programming = Rc::new(Node {
           value: "programming",
            parent: Some(Rc::clone(&root)),
        });

        let rust = Rc::new(Node {
            value: "rust",
            parent: Some(Rc::clone(&programming)),
        });

        let math = Rc::new(Node {
            value: "math",
            parent: Some(Rc::clone(&root)),
        });

        let algo = Rc::new(Node {
            value: "algo",
            parent: Some(Rc::clone(&math)),
        });

        for node in vec![root, programming, rust, math, algo] {
            let mut queue = vec![&node];
            let mut current = queue.pop().unwrap();
            while let Some(node) = current.parent.clone() {
                queue.push(&node);
                current = &node;
            }
            println!("node: {}", node.value);
        }
    }
}

fn eg_traits() {
    let wizard = Wizard;
    wizard.fly();
    wizard.spell();
    let lizard = Lizard;
    lizard.swim();
    let warrior = Warrior;
    warrior.punch();
    println!("Warrior level up and now can punch and spell!");
    warrior.spell();
}

fn eg_shadowing() {
    let mut b: usize = 10;
    println!("b value: {} location: {:p}", b, &b);
    b = 5;
    println!("b value: {} location: {:p}", b, &b);
    let mut b: usize = 4;
    println!("b value: {} location: {:p}", b, &b);
    b = 3;
    println!("b value: {} location: {:p}", b, &b);
}

fn eg_immutable_reference() {
    let mut x: i32;
    x = 43;
    assert_eq!(x, 43);
    println!("x: {x}");

    let y = &x;
    println!("x: {}", *y);
    x = 23;
    assert_eq!(x, 23);
    println!("x: {}", x);
    // assert_eq!(*y, 23); // error: cannot assign to immutable reference
    let id = Identity::id(&x);
    println!("idx: {}", id);
    println!("idx2: {}", x.id());
}

trait Flying {
    fn fly(&self);
}

trait Punching {
    fn punch(&self);
}

trait Spelling {
    fn spell(&self);
}

trait Swim {
    fn swim(&self);
}

struct Wizard;

impl Flying for Wizard {
    fn fly(&self) {
        println!("Wizard flies");
    }
}
impl Spelling for Wizard {
    fn spell(&self) {
        println!("Wizard spells");
    }
}

struct Warrior;

impl Punching for Warrior {
    fn punch(&self) {
        println!("Warrior punches");
    }
}

impl Spelling for Warrior {
    fn spell(&self) {
        println!("Warrior spells");
    }
}


struct Lizard;
impl Swim for Lizard {
    fn swim(&self) {
        println!("Lezard swims");
    }
}


trait Identity<T> {
    fn id(&self) -> T;
}

impl<T: Copy> Identity<T> for T {
    fn id(&self) -> T {
        *self
    }
}
