# The Anon programming language

The Anon programming language is designed to be easy and elegant. We put utility and unification first in order to decrease the difficulties to use, read and remember.

## The features of Anon

* Immutable is better than mutable
* No effect should be hidden
* Type does not imply the memory layout
* Name is important
* The behaviors of an object defines itself
* Readability and maintainability are important
* We will do want you need to do but don't want to
* Leave the 1% edge case for us to get 200% of efficiency for you

## Example code

print hello world

``` ignore
-- hello-world.an
main :: () -> () with Out Console
main = print "hello world!"
```

interact with the real world

```ignore

```

use refinement types for concision

```ignore
-- refinement-types.an
refine EmptyIntVec = Vec Int where Vec.len = 0
refine OneElementIntVec = Vec Int where Vec.len = 1

sumOfVec :: Vec Int -> Int
sumOfVec EmptyIntVec = 0
sumOfVec (x :: OneElementIntVec) = x[0]
sumOfVec x = [0] + sumOfVec x[1..]

```

define data structure with ease

``` ignore
-- data.an

type Maybe a = data a {
  Just a | None
}
can {
  unwrap :: Self -> a with Maybe Panic!
  unwrap self = 
    match self
      Just x -> a
      Nil -> panic! "Unwrapping Nil!"
      
-- You can also write unwrap like this
-- unwrap :: Self -> a with Maybe Panic!
-- unwrap Just a = a
-- unwrap Nil = panic! "Unwrapping Nil!"
}

type List = data a {
  Cons a Self | Nil
}
can {
  unwrap :: (Self) -> Maybe a
  unwrap self = 
    match self
      Cons a _ -> Just a
      Nil -> Nothing
  
  get :: (Self, Usize) -> Maybe a
  get self 0 = unwrap self
  get self index = 
    match self
      Cons x -> unwrap $ get x index - 1
      Nil -> Nothing
}
```

prevent unwanted/harmful effects

``` ignore
badAdd = 

```
