# Part 3

Now that we're comfortable with the basics of Rust synatx and semantics, we'll
pick things up a little bit by adding in more advanced things like
multithreading.

Rust ensures that our code is free of data races. Sometimes this means that we
explictly have to wrap objects in syncronization primitives to make sure that data
is not suceptible to data races.

## The Challenge

We'll be making a thread pool which allows us to do work on many different threads,
reusing them after their previous work is done.

```rust
  let pool = ThreadPool::new(4);
  let count = Arc::new(AtomicUsize::new(0));

  let count1 = count.clone();
  pool.execute(move || {
      println!("Thread 1");
      count1.fetch_add(1, Ordering::SeqCst);
      std::thread::sleep(std::time::Duration::from_secs(1));
  });
  let count2 = count.clone();
  pool.execute(move || {
      println!("Thread 2");
      count2.fetch_add(1, Ordering::SeqCst);
      std::thread::sleep(std::time::Duration::from_secs(1));
  });
  let count3 = count.clone();
  pool.execute(move || {
      println!("Thread 3");
      count3.fetch_add(1, Ordering::SeqCst);
      std::thread::sleep(std::time::Duration::from_secs(1));
  });
  let count4 = count.clone();
  pool.execute(move || {
      println!("Thread 4");
      count4.fetch_add(1, Ordering::SeqCst);
      std::thread::sleep(std::time::Duration::from_secs(1));
  });
  std::thread::sleep(std::time::Duration::from_secs(2));

  let count = count.load(Ordering::SeqCst);

  assert_eq!(count, 4);
```

It's important to note how we are forced by the Rust compiler to use an Arc and
an AtomicUsize in order to add to the count in a thread safe way.

## Hints

Closure traits are special and have been a part of Rust since 1.0. Because of this,
we cannot directly call boxed closures in a straight forward way. We'll need to
define a wrapper trait that allows us to do this similiar to the following:

```rust
trait BoxedFn {
    fn call(self: Box<Self>);
}

impl<F: FnOnce()> BoxedFn for F {
    fn call(self: Box<F>) {
        (*self)()
    }
}
```

Inter-thread communication is often done in Rust using channels. Take a look at the
[mspc](https://doc.rust-lang.org/std/sync/mspc/index.html) docs to learn more about
this.
