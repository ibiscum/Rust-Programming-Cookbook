//!
//! A simple singly-linked list for the Rust-Cookbook by Packt Publishing. 
//! 
//! Recipes covered in this module:
//!  - Documenting your code
//!  - Testing your documentation
//!  - Writing tests and benchmarks
//! 


#![cfg_attr(feature = "nightly", feature(test))]
#![doc(html_logo_url = "https://blog.x5ff.xyz/img/main/logo.png",
       test(no_crate_inject, attr(allow(unused_variables), deny(warnings))))]

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> where T: Sized + Clone  {
    value: T,
    next: Link<T>,
}


impl<T> Node<T>  where T: Sized + Clone  {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

/// 
/// A singly-linked list, with nodes allocated on the heap using `Rc`s and `RefCell`s. Here's an image illustrating a linked list:
/// 
/// 
/// ![](https://upload.wikimedia.org/wikipedia/commons/6/6d/Singly-linked-list.svg)
/// 
/// *Found on https://en.wikipedia.org/wiki/Linked_list*
/// 
/// # Usage
/// 
/// ```ignore
/// let list = List::new_empty();
/// ```
/// 
#[derive(Clone)]
pub struct List<T>  where T: Sized + Clone {
    head: Link<T>,
    tail: Link<T>,
    
    /// 
    /// The length of the list.
    /// 
    pub length: usize,
}

impl<T> List<T> where T: Sized + Clone  {

    ///
    /// Creates a new empty list.
    /// 
    ///  
    /// # Example
    /// 
    /// ```
    /// # use testing::List;
    /// let list: List<i32> = List::new_empty();
    /// ```
    /// 
    pub fn new_empty() -> List<T> {
        List { head: None, tail: None, length: 0 }
    }

    ///
    /// Appends a node to the list at the end.
    /// 
    ///  
    /// # Panics
    /// 
    /// This never panics (probably).
    /// 
    /// # Safety
    /// 
    /// No unsafe code was used.
    /// 
    /// # Example
    /// 
    /// ```
    /// use testing::List;
    /// 
    /// let mut list = List::new_empty();
    /// list.append(10);
    /// ```
    /// 
    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()), 
            None => self.head = Some(new.clone())
        };    
        self.length += 1;
        self.tail = Some(new);
    }

    ///
    /// Removes the list's head and returns the result. 
    /// 
    ///  
    /// # Panics
    /// 
    /// Whenever when a node unexpectedly is `None`
    /// 
    /// # Example
    /// 
    /// ```
    /// # use testing::List;
    /// 
    /// let mut list = List::new_empty();
    /// list.append(10);
    /// assert_eq!(list.pop(), Some(10));
    /// ```
    /// 
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}

impl <T>Drop for List<T> where T: Clone + Sized {

    fn drop(&mut self) {
        while self.length > 0 {
            let n = self.pop();
            drop(n);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "nightly")]
    extern crate test;

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_list_append(b: &mut test::Bencher) {
        let mut list = List::new_empty();
        b.iter(|| {
            list.append(10);
        });
    }

    #[test]
    fn test_list_new_empty() {
        let mut list: List<i32> = List::new_empty();
        assert_eq!(list.length, 0);
        assert_eq!(list.pop(), None);
    }       

    #[test]
    fn test_list_append() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);     
        list.append(1);
        assert_eq!(list.length, 5);
    }


    #[test]
    fn test_list_pop() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.length, 0);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_single_element() {
        // Regression: a single-element list should handle append and pop correctly.
        let mut list = List::new_empty();
        list.append(42);
        assert_eq!(list.length, 1);
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_list_pop_empty() {
        // Regression: popping from an empty list should always return None.
        let mut list: List<i32> = List::new_empty();
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_list_strings() {
        // Regression: the list should work for non-Copy types such as String.
        let mut list = List::new_empty();
        list.append("hello".to_string());
        list.append("world".to_string());
        assert_eq!(list.pop(), Some("hello".to_string()));
        assert_eq!(list.pop(), Some("world".to_string()));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_interleaved_append_and_pop_fifo() {
        // Regression: interleaving operations should preserve FIFO ordering.
        let mut list = List::new_empty();
        for i in 1..=5 {
            list.append(i);
        }
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.append(6);
        list.append(7);
        assert_eq!(list.length, 5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_list_drop_clears_nodes() {
        // Regression: dropping a list should not leak nodes.
        {
            let mut list = List::new_empty();
            for i in 0..100 {
                list.append(i);
            }
            assert_eq!(list.length, 100);
        }
        // If Drop panicked or leaked, we would not reach this assertion.
        assert!(true);
    }
}
