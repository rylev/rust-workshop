# Rust Workshop

This is a Rust workshop specifically focused on those with no experience in
Rust, but with at least some experience in systems programming either through C
or C++.

## Getting Started

Installing Rust should hopefully be relatively straight forward, but if you run into
any issues, please do let me know so I can help troubleshoot.

To install the toolchain, first go to [rustup.rs](https://rustup.rs), and follow the
instructions. This should install the compiler, Cargo (Rust's build tool), and
other things like documentation.

You can check that this all worked by:
* opening your terminal (PowerShell, cmd.exe, bash, etc. should all work)
* entering `cargo new hello-world`
* changing into the newly created `hello-world` directory
* running `cargo run`
* if you see "Hello, world!" printed to your screen, then everything should be good!

I recommend VS Code as the programming environment for Rust though there are relatively
sizable portions of the community that use vim, emacs, Sublime, and IntelliJ. Using
Rust in Visual Studio is possible, but it might be a bit buggy.

If you use VS Code, make sure to install the Rust extension and open your hello-world
project in VS Code. You should then be prompted to install the Rust language server (RLS).
Once the RLS is installed, your environment should be ready.

Once you have a Rust environment set up, go to [part 1](./part-1) folder to get
started.

## Additional Resources

* [The Book](https://doc.rust-lang.org/book/): _The_ resource for learning Rust
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/): Learning Rust by going through examples
* [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/): A great way to push yourself to understand Rust's borrower checker and memory model
* [Learn Rust the Dangerous Way](http://cliffle.com/p/dangerust/): For those who really want to understand what is happening with Rust compared to C, this is a great resource.
* [Jon Gjengset’s Rust Streams](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ): The best intermediate to advanced long form content on Rust
* [Ryan's Learn Rust Stream](https://www.youtube.com/watch?v=DWNyZXUC1u4): An early version of part-1 of this workshop
