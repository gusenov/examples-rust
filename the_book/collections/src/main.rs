#![allow(unused)]

// Rust’s standard library includes a number of very useful data structures called collections. 
// Most other data types represent one specific value, but collections can contain multiple values. 
// Unlike the built-in array and tuple types, the data that these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. 
// Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. 

fn main() {
    // A vector allows you to store a variable number of values next to each other.

    // create a new, empty vector
    let v: Vec<i32> = Vec::new();

    // Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.
    let v = vec![1, 2, 3];

    // create a vector and then add elements to it
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];
    
    // Using & and [] gives us a reference to the element at the index value. 
    // vectors are indexed by number, starting at zero.
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    
    // When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    let v = vec![1, 2, 3, 4, 5];

    // When we run this code, the first [] method will cause the program to panic because it references a nonexistent element. 
    // This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
    //let does_not_exist = &v[100];
    
    // When the get method is passed an index that is outside the vector, it returns None without panicking. 
    // You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. 
    // Your code will then have logic to handle having either Some(&element) or None
    let does_not_exist = v.get(100);


    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // you can’t have mutable and immutable references in the same scope.
    //v.push(6);
    // Why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: 
    // Because vectors put the values next to each other in memory, 
    // adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, 
    // if there isn’t enough room to put all the elements next to each other where the vector is currently stored. 
    // In that case, the reference to the first element would be pointing to deallocated memory. 
    // The borrowing rules prevent programs from ending up in that situation.
    println!("The first element is: {first}");

    // To access each element in a vector in turn, we would iterate through all of the elements
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
        *i += 50;
    }

    // Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules.

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object

    // Like any other struct, a vector is freed when it goes out of scope
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
    // When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. 
    // The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.




    // A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter, we’ll talk about it in depth.

    // create an instance
    let mut s = String::new();

    let data = "initial contents";
    // to_string method, which is available on any type that implements the Display trait, as string literals do.
    let s = data.to_string();
    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    // use the function String::from to create a String from a string literal.
    let s = String::from("initial contents");

    // String::from and to_string do the same thing, so which one you choose is a matter of style and readability.

    // strings are UTF-8 encoded, so we can include any properly encoded data in them
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // append a string slice
    let mut s = String::from("foo");
    // The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter.
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push method takes a single character as a parameter and adds it to the String
    let mut s = String::from("lo");
    s.push('l');

    // combine two existing strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {

    // concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    // For combining strings in more complicated ways, we can instead use the format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    let s1 = String::from("hi");
    //let h = s1[0];
    // Rust strings don’t support indexing.

    // len will be 4, which means the vector storing the string "Hola" is 4 bytes long.
    let hello = String::from("Hola");

    // each Unicode scalar value in that string takes 2 bytes of storage. 
    // Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.
    let hello = String::from("Здравствуйте");

    // Here, s will be a &str that contains the first 4 bytes of the string. 
    // Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд.
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // Iterating Over Strings

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }




    // A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

    // Just like vectors, hash maps store their data on the heap. 
    // Like vectors, hash maps are homogeneous: All of the keys must have the same type, and all of the values must have the same type.

    // One way to create an empty hash map is to use new and to add elements with insert.
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get a value out of the hash map by providing its key to the get method
    let team_name = String::from("Blue");
    // This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, 
    // then unwrap_or to set score to zero if scores doesn’t have an entry for the key.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iterate over each key-value pair
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    // For owned values like String, the values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // If we insert references to values into the hash map, the values won’t be moved into the hash map.
    // The values that the references point to must be valid for at least as long as the hash map is valid.

    // If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced. 
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // Hash maps have a special API for this called entry that takes the key you want to check as a parameter. 
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist. 
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value. 
    // This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.
    println!("{scores:?}");

    // look up a key’s value and then update it based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // The split_whitespace method returns an iterator over subslices, separated by whitespace, of the value in text. 
    for word in text.split_whitespace() {
        // The or_insert method returns a mutable reference (&mut V) to the value for the specified key. 
        // Here, we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). 
        // The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    // If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. 
    // A hasher is a type that implements the BuildHasher trait.

}
