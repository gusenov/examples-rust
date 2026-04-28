#![allow(unused)]

fn main() {
    // Creating an iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // Using an iterator in a for loop
    for val in v1_iter {
        println!("Got: {val}");
    }




    // Methods That Produce Other Iterators
    let v1: Vec<i32> = vec![1, 2, 3];
    
    // The map method returns a new iterator that produces the modified items.
    v1.iter().map(|x| x + 1);  // doesn’t do anything; the closure we’ve specified never gets called.
    // Iterator adapters are lazy, and we need to consume the iterator here.

    // collect the results of iterating over the iterator that’s returned from the call to map into a vector. 
    // This vector will end up containing each item from the original vector, incremented by 1.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
// note that the values we get from the calls to next are immutable references to the values in the vector. 
// The iter method produces an iterator over immutable references. 
// If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. 
// Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

// The Iterator trait has a number of different methods with default implementations provided by the standard library; you can find out about these methods by looking in the standard library API documentation for the Iterator trait. 
// Some of these methods call the next method in their definition, which is why you’re required to implement the next method when implementing the Iterator trait.
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
// We aren’t allowed to use v1_iter after the call to sum, because sum takes ownership of the iterator we call it on.
