#![allow(unused)]

// smart pointers implement the Deref and Drop traits. 
//  - The Deref trait allows an instance of the smart pointer struct to behave like a reference so that you can write your code to work with either references or smart pointers. 
//  - The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

// most common smart pointers in the standard library:
//  - Box<T>, for allocating values on the heap
//  - Rc<T>, a reference counting type that enables multiple ownership
//  - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time




enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};




// The Box<T> type is ultimately defined as a tuple struct with one element
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}




fn hello(name: &str) {
    println!("Hello, {name}!");
}




struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}




enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}
use crate::List2::{Cons2, Nil2};
use std::rc::Rc;




enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}
use crate::List3::{Cons3, Nil3};
use std::cell::RefCell;




fn main() {

    // how to use a box to store an i32 value on the heap
    let b = Box::new(5);
    println!("b = {b}");




    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));



    let x = 5;
    assert_eq!(5, x);
    // Using the dereference operator to follow a reference to an i32 value
    let y = &x;
    assert_eq!(5, *y);
    // Using the dereference operator on a Box<i32>
    let y = Box::new(x);
    assert_eq!(5, *y);




    let y = MyBox::new(x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));




    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value. 
    // Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref. 
    // The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref. 
    // Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
    
    hello(&(*m)[..]);
    // The (*m) dereferences the MyBox<String> into a String. 
    // Then, the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello.

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    //  - From &T to &U when T: Deref<Target=U>
    //  - From &mut T to &mut U when T: DerefMut<Target=U>
    //  - From &mut T to &U when T: Deref<Target=U>




    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    // Calling std::mem::drop to explicitly drop a value before it goes out of scope
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");




    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));




    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));
    let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    //println!("a after = {a:?}");
    //println!("b after = {b:?}");
    //println!("c after = {c:?}");

}
