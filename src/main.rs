use std::rc::Rc;

use rust_smart_ptrs::{
    hello, CustomSmartPointer,
    List::{Cons, Nil},
    MyBox,
};

fn main() {
    let cons_list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    println!("{:?}", cons_list);

    let x = 5;

    let y = Box::new(&x);

    assert_eq!(5, **y);

    let a = 5;

    let b = MyBox::new(&a);

    assert_eq!(5, **b);

    let m = MyBox::new(String::from("Rust"));

    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    drop(c);

    println!("CustomSmartPointers created.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
