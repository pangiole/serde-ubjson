# serde-ubj
[Universal Binary JSON](https://github.com/ubjson/universal-binary-json) data format for Rust with [Serde](https://github.com/serde-rs/serde) (both `std` and `no_std` environments).

<div class="warning">
<p>This project is in an early stage of development, and <strong>not</strong> production ready yet.<br>Use with caution!</p>
</div>

This project does:

* implement the [`serde::Serializer`](https://docs.rs/serde/1.0.228/serde/trait.Serializer.html) trait for most of the [Serde data model](https://serde.rs/data-model.html) types,
* support optimized `serialize_bytes` (that you can enable via the [`serde_bytes`](https://github.com/serde-rs/bytes) crate),
* support both `std` (standard) and `no_std` environments (by replying upon the [`embedded-io`](https://github.com/rust-embedded/embedded-hal/tree/master/embedded-io) project),
* pass enough unit tests covering at least 85% of its code base.


## tl;dr
For the impatient.

### serialization
With any Rust `std` writer of your choice (i.e., output console, memory buffer, file on disk, network socket, etc.), serialize any value for which you have either derived 
or provided a `serde::Serialize` implementation to Universal Binary JSON in a few instructions: 

```rust,ignore
use sdt::{error, io};

fn main() -> Result<(), Box<dyn error::Error>> {

  let value = 123_i16;
  // let value = ... my model type ... 
  
  // With any IO writer, serialize the value
  let mut w: io::Write = io::stdout();
  serde_ubj::to_writer(&w, &value)?;
}
```

### deserialization
Whit any Rust `std` buffered reader of your choice (i.e., input console, memory buffer, file on disk, network socket, etc.), deserialize any value for which you have either derived or provided `serde::Deserialize` implementation from Universal Binary JSON in a few instructions:

```rust,ignore
use sdt::{error,io}

fn main() -> Result<(), Box<dyn error::Error>> {
  // With any IO buffered reader,
  let mut r: io::BufRead = ...;
  
  // Deserialize the value of type `T`
  let value: T = serde_ubj::from_buf_reader(&mut r)?;
  Ok(())
}
```

## exceptions
This implementation does **not** support the following Serde types:

* **serialization**
  * Serde byte array
  * Serde numeric `u64` values greater than Rust `i64::MAX`
  * Serde numeric `i128`, `u128`
  * Serde `string` having length greater than Rust `i64::MAX`,
  * UBJ optimizations

* **deserialization**
  * all exceptions above, and
  * Serde `u16`, `u32`, `u64`
  * Serde `&str`

## limitations
This implementation is made with the following limitations:

* **strings**  
  their length must be not greater than `i64::MAX`

* **maps**  
  keys must be `String`s

## no_std
This implementation can be adopted in `no_std` environments, as long as you disable the default features and, at the same time, enable the `embedded-io` feature, as per following `Cargo.toml` example:

```toml
[package]
name = "my_no_std_project"
version = "0.1.0"
edition = "2024"

[dependencies]
serde_ubj = { version = "0.2.0", default-features = false, features = ["embedded-io"] }
embedded-io = { version = "latest" }
```

## book
Coming soon.