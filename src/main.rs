use rust_smart_ptrs::{
    hello, CustomSmartPointer,
    List::{Cons, Nil},
    MyBox,
};

fn main() {
    let cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

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
}
