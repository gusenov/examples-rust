#![allow(unused)]

// Every programming language has tools for effectively handling the duplication of concepts. 
// In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties. 
// We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

// lifetimes: a variety of generics that give the compiler information about how references relate to each other. 
// Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure that references will be valid in more situations than it could without our help.




fn largest_(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}




fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}




fn largest__<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}




struct Point<T> {
    x: T,
    y: T,
}




struct Point_<T, U> {
    x: T,
    y: U,
}




struct Point__<T> {
    x: T,
    y: T,
}

impl<T> Point__<T> {
    fn x(&self) -> &T {
        &self.x
    }
}




impl Point__<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}




struct Point___<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point___<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point___<X2, Y2>) -> Point___<X1, Y2> {
        Point___ {
            x: self.x,
            y: other.y,
        }
    }
}




fn main() {
    // finds the largest number in a list
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");




    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_(&number_list);
    println!("The largest number is {result}");




    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");




    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest__(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest__(&char_list);
    println!("The largest char is {result}");




    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };




    let both_integer = Point_ { x: 5, y: 10 };
    let both_float = Point_ { x: 1.0, y: 4.0 };
    let integer_and_float = Point_ { x: 5, y: 4.0 };




    let p = Point__ { x: 5, y: 10 };
    println!("p.x = {}", p.x());




    let p1 = Point___ { x: 5, y: 10.4 };
    let p2 = Point___ { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
