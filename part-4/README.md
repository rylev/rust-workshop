# Part 2

Now that we're confident writing code in Rust, we're next going to try integrating some non-Rust components with our Rust code.

## The Challenge

Let's use the [`libcurl`](https://curl.haxx.se/libcurl/c/libcurl.html) library to perform an HTTPS request.

In order to call libcurl's C API, we'll first need to define libcurl's data types and fuction signatures in Rust.
Fortunately, the [`curl-rust`](https://github.com/alexcrichton/curl-rust) project has already done this for us in curl-sys.

```rust
    let mut curl = Curl::new();
    curl.url("https://www.rust-lang.org/").unwrap();
    curl.perform().unwrap();
```

## A Few Hints

We'll need to specify a dependency on the curl-sys package from [`curl-rust`](https://github.com/alexcrichton/curl-rust). Cargo provides nice semantics to include
dependencies from github repositories. Learn about them with the [`Cargo Book`](https://doc.rust-lang.org/cargo/index.html).

Libcurl provides an easy C interface to do a file transfer with [`curl_easy_perform`](https://curl.haxx.se/libcurl/c/curl_easy_perform.html).
To write the output to the console, check out [`CURLOPT_WRITEFUNCTION`](https://curl.haxx.se/libcurl/c/CURLOPT_WRITEFUNCTION.html).

If there isn't a Rust interface readily available for you, you can define the interface in Rust yourself, or just let [`bindgen`](https://rust-embedded.github.io/book/interoperability/c-with-rust.html) do it for you.
