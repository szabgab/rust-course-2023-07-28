# Experimental Rust Course in Hebrew

Started on 2023-07-28

## Videos

* [PlayList](https://www.youtube.com/playlist?list=PLm2NBp4tb5F0GfrV8DSxwDn2I88D3gcc7)

## Session 1

* [slides](https://code-maven.com/slides/rust/)

* Sales-pitch - why learn Rust?

* Speed - on par with C and C++ ??
* Memory safety

* Strong static type system
* No garbage collection


Open source projects that were written in Rust, but are used by not necessarily Rust programmers.

* [Slint](https://slint.dev/) GUI design language.
* [SurrealDB](https://surrealdb.com/) multi-model database.
* [Fish Folk](https://fishfolk.org/) game.
* [Foundry](https://github.com/foundry-rs/foundry) toolkit for Ethereum application development.
* [Lemmy](https://lemmy.world/) is a free, open source, distributed alternative to Reddit. [GitHub](https://github.com/LemmyNet/lemmy).

* [MeiliSearch](https://www.meilisearch.com/)
* Components of [Firefox](https://www.mozilla.org/en-US/firefox/)
* In the [Linux kernel](https://docs.kernel.org/rust/index.html)  [kernel](https://git.kernel.org/) Elixir cross reference for [rust](https://elixir.bootlin.com/linux/v6.4.7/source/rust)
* [coreutils](https://github.com/uutils/coreutils/)

* [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
* [Awesome Alternatives in Rust](https://github.com/TaKO8Ki/awesome-alternatives-in-rust)
* [Popular Rust projects](https://github.com/search?q=stars%3A%3E13000+language%3ARust&type=Repositories&ref=advsearch&l=&s=stars&o=desc)

* [Firecracker](https://firecracker-microvm.github.io/)
* [Tauri](https://tauri.app/)
* [Tokio](https://tokio.rs/) is an asynchronous runtime for the Rust programming language.
* [Martin](https://martin.maplibre.org/) - PostGIS, MBtiles and PMtiles tile server [GitHub](https://github.com/maplibre/martin). (GeoSpatial field)
* [t-rex](https://t-rex.tileserver.ch/) - is a vector tile server specialized on publishing MVT tiles from your own data. [GitHub](https://github.com/t-rex-tileserver/t-rex/). (GeoSpatial field)

* [Deno](https://deno.land/) JavaScript runtime.

* [Ruff](https://beta.ruff.rs/docs/) a very fast Python linter.
* [Polars](https://www.pola.rs/) Lightning-fast DataFrame library for Rust and Python.
* [Pydantic](https://docs.pydantic.dev/latest/) data validation library for Python.

* Install Rust

* Editor:
    * Vim + ??
    * VScode + rust-analyzer

* `cargo new demo`

* variable `name` with fixed string, print string

* println! print!   eprintln!   eprint!  dbg!  format!  (macros, metaprogramming)

* Exercise: install, hello world, create a variable with your name and print it too.


* variable with numbers
* types of numbers: i8, u8, f32 ... default is i32 or f64 respectively.
* operations with numbers
* converting integer to floating point with  `var as f32`

* function declaration passing numbers, returning numbers `add()`

* Exercise: function that gets two numbers and returns their sum (what I showed)
* Exercise: 3 other functions for the 3 other operations

* `if` statement

* `for` loop on range `for i in 1..5 {}`, break , continue

* mutable variables

* shadowing variables (redeclare)

* `match`

* Exercise: calculator function that gets 2 numbers and an operator `+,-,*,/` and returns the result
* Exercise: factorial function (given a number return n!)
* Exercise: fibonacci function (given a number N return the Nth fibonacci number)

* [Video-0](https://youtu.be/ElF1wO7ZO-I)
* [Video-1](https://youtu.be/zc2Ey0miHG4)
    * 00:00 Start
    * 00:58 Course Plan
    * 01:30 Self-Introduction
    * 04:20 Why use Rust? Course Resources.
    * 06:40 Speed
    * 07:50 Memory Safety / No Garbage Collection
    * 09:35 Strong Static Types
    * 11:50 Open-Source Projects
    * 12:42 Memory Management
    * 15:12 Course Resources
    * 16:20 Rust Installation
    * 17:10 Hello World project
    * 36:10 Exercises: Install, Hello World

* [Video-2](https://youtu.be/Kml0GypwX2Q)
    * 00:00 Rust within VS Code
    * 04:10 Numbers project
    * 14:30 Functions
    * 18:30 Return Value - Expression
    * 22:55 Numbers - Exercises
    * 23:45 Open-Source Projects in Rust
    * 30:30 3rd Party Libraries

* [Video-3](https://youtu.be/JGwTaWw0oRw)
    * 00:52 `if` statement
    * 02:40 `if` expression
    * 06:25 `for` loop
    * 08:15 Variables project
    * 09:00 mutable - immutable
    * 18:15 Result type
    * 28:10 `match`
    * 34:10 Exercises: Calculator, Factorial, Fibonacci
    * 35:50 Macros
    * 37:45 Error Handling Demo


### Projects (folders) used

* [hello-world](hello-world)
* [numbers](numbers)
* [variables](variables)


## Session 2


