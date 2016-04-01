//http://cglab.ca/~abeinges/blah/too-many-lists/book/

#![allow(dead_code)]
use std::rc::Rc;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum List<T> {
  Nil,
  Cons(Cons<T>)
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Cons<T> {
  hd: T,
  tl: Rc<List<T>>,
}

pub struct Iter<'a, T:'a> {
  next: Option<&'a Cons<T>>,
}

pub fn push<T>(list: List<T>, item: T) -> List<T> {
  List::Cons(Cons{hd: item, tl:Rc::new(list)})
}

// fold : S list -> T -> (T -> S -> T) -> T

pub fn fold<T,S,F>(list: &List<S>, accum:T, f:F) -> T
  where F:Fn(T, &S) -> T {
    match *list {
      List::Nil => accum,
      List::Cons(ref cons) => {
        let accum2 = f(accum, &cons.hd);
        return fold(&*cons.tl, accum2, f)
      }
    }
}

#[test]
pub fn test_sum() {
  let l = List::Nil;
  let l = List::Cons(Cons{hd:3, tl:Rc::new(l)});
  let l = List::Cons(Cons{hd:2, tl:Rc::new(l)});
  let l = List::Cons(Cons{hd:1, tl:Rc::new(l)});
  let sum = fold(&l, 0, |sum,elm|{ sum + elm });
  assert_eq!(sum, 6);    
}

#[test]
pub fn test_push() {
  let l = List::Nil;
  let l = push(l, 3);
  let l = push(l, 2);
  let l = push(l, 1);
  let sum = fold(&l, 0, |sum, elm|{ sum + elm });
  assert_eq!(sum, 6);
}