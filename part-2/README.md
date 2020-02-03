# Part 2

The second part of the workshop will focus on taking on our knowledge of the Rust's
type system, standard library, and borrow checker to create a fluent API.

We'll be creating a StringStore that allows us to reuse strings that we've already
created to reduce the amount of times we need to allocate memory. This can be useful
if we expeect to have many short-lived Strings where the cost of allocation might
start affecting performance.

## The Challenge

We'll be creating a StringStore struct that allows us to allocate a certain number of
strings with a certain preset capacity. We can get a "string" from the story and when
the "string" is dropped and we get another string, the underlying memory is reused.

```rust
  let store = StringStore::with_capacity(1, 3);
  let mut string = store.get();

  string.push_str("Wow");
  drop(string);

  let string = store.get();

  assert_eq!(string.len(), 0);
```

## A Few Hints

We'll be using the "guard" pattern where-by when we borrow the "string" from the
store, we actually get a wrapper struct that return the string to the store when the
guard is dropped.

In order to keep the API easy to use, we can use the [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
and [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) traits, to
make the guard objects feel like they're just plain strings.
