# Part 1

The first part of the workshop will be focused on getting a grasp with the basic
syntax and semantics of Rust.

In order to do this, we'll first be creating a hashmap from scratch. This
exercise is a great way to get comfortable with the syntax of Rust as well as
some of the basic ideas of Rust semantics.

## The Challenge

We'll be creating a very simple hashmap that should be at least in the same
ballpark performance wise as the standard library implementation. The interface
our hashmap will expose is very simple:

```rust
  let mut map = HashMap::new();
  assert_eq!(map.insert("foo", "bar"), None);
  assert_eq!(map.insert("foo", "lol"), Some("bar"));

  assert_eq!(map.get(&"foo"), Some(&"lol"));
  assert_eq!(map.get(&"foo"), Some(&"lol"));
  assert_eq!(map.get(&"qux"), None);

  assert_eq!(map.remove(&"foo"), Some("lol"));
  assert_eq!(map.get(&"foo"), None);
```

To summarize, the hashmap is only capable of being initialized, inserting key/value
pairs, getting a value based on a key, and removing a value based on a key.

If you're unfamiliar with hashmaps, they're actually quite simple. Each hashmap
has a distinct number of buckets which are just simply just collections of
key/value pairs. When inserting into a hashmap, a hashing function is run on the
key to determine which bucket it falls into. While hashes on different keys
should be unique, given that we'll have a very low number of buckets, it's
common for different keys to end up leading to the same bucket. When this
happens, we simply just push the key/value pair on to the collection.

When fetching a value based on a key, the hash function is computed again, the
bucket is retrieved for that hash value and then a linear such is performed on
the bucket to find the value.
