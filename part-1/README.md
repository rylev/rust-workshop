# Part 1

The first part of the workshop will be focused on getting a grasp of the basic
syntax and semantics of Rust.

In order to do this, we'll first be creating a HashMap from scratch. This
exercise is a great way to get comfortable with the syntax of Rust as well as
some of the basic ideas of Rust semantics.

## The Challenge

We'll be creating a very simple hashmap that should be at least in the same
ballpark performance wise as the standard library implementation. The interface
our hashmap will expose is very simple:

```rust
  // create a new HashMap
  let mut map = HashMap::new();

  // insert key/value pairs into the HashMap
  assert_eq!(map.insert("foo", "bar"), None);
  // if an item already exists for that value, it should return the old value
  assert_eq!(map.insert("foo", "lol"), Some("bar"));

  // get a value based on its key
  assert_eq!(map.get(&"foo"), Some(&"lol"));
  // you should be able to do this multiple times
  assert_eq!(map.get(&"foo"), Some(&"lol"));
  // if no value exists for a key, return None
  assert_eq!(map.get(&"qux"), None);

  // remove a value for a key
  assert_eq!(map.remove(&"foo"), Some("lol"));
  // once it no longer exists in the map, it should return None
  assert_eq!(map.get(&"foo"), None);
```

To summarize, the HashMap is only capable of being initialized, inserting key/value
pairs, getting a value based on a key, and removing a value based on a key.

### How HashMaps actually work

If you're unfamiliar with hashmaps, they're actually quite simple. Each HashMaps
contains a collection of "buckets". These buckets are themselves collections of
key/value pairs. When inserting into a HashMap, a hashing function is run on the
key to determine which bucket it falls into.

While the hashes on different keys should be unique, the HashMap will have a limited
number of buckets (e.g., 16). So when reducing between a hash (a 64 bit number) and
a bucket index a number between 0 and (NUM_BUCKETS - 1), many different hashes will
lead to the same bucket. When this happens, we simply just push the key/value pair
on to the collection.

When fetching a value based on a key, the hash function is computed again, the
bucket is retrieved for that hash value and then a linear search is performed on
the bucket to find the value.

### Checkpoint 1

First, start a new project with `cargo new my-hashmap --lib`. This will create a
new Cargo project with everything we need to get started. Changed into your project's
directory and open up `lib.rs`.

That file should look something like this:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Replace the contents of `it_works` with the HashMap code above. We'll use this code
to test that our HashMap works.

You'll also need to add the line `use super::*;` above `#[test]` to bring the code
we write into the `tests` module namespace.

Try running `cargo test` and hopefully you should see an error about an undeclared
type or module named `HashMap`. If you're seeing any errors about syntax you've
made the wrong move.

### Checkpoint 2

We'll now comment out using `//` all the lines of our test but the first, and try
to get the test compiling and passing.

### Checkpoint 3

Keep uncommenting each line of code in the test and try to get the tests compling.
They won't pass for now (unless you've gone ahead and implemented a `HashMap`!),
but it should be possible to get them to compile.

Hint: You can use the `todo!()` or `unimplemeneted!()` macros to help you get your
code to compile. Try using these in your function bodies instead of actually
implementing the HashMap for now.

### Checkpoint 4

Think about how to model your buckets and try implementing the `get` function.

Hint: you'll want to take a look at the [`Hasher`](https://doc.rust-lang.org/std/hash/trait.Hasher.html)
and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) traits as well as
the [`DefaultHasher`](https://doc.rust-lang.org/std/collections/hash_map/struct.DefaultHasher.html)
struct from the standard library.

### Checkpoint 5

Try implementing the other two methods `insert` and `delete`.

### Checkpoint 6

Can you make the HashMap generic? You'll need to make sure that whatever generic types
your struct takes that you constrain them so that you know they have specific capabilities
like equality and the ability to be hashed.

### Extra credit

Once you've gotten this far, you should have a decent understanding of Rust basics.
If you want to go further, try recreating the [`entry` api](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
from the standard library.
