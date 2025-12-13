# serde-ubjson
The [UBSJON](https://github.com/ubjson/universal-binary-json) format implementation for Rust with [Serde](https://github.com/serde-rs/serde)

## tl;dr
With any writer you like, serialize your model to UBJSON in a few instructions: 

```rust
use serde_ubjson;
use sdt::{error, io};

fn main() -> Result<(), Box<dyn error::Error>> {

  // With any writer you like  
  let mut w: io::Write = ...;
  // for example stdout, buffer, file, socket, etc.

  // Create your user-defined data model
  let model: MyModel = ...; 
  // for which you have derived the serde::Serialize implementation   
    
  // And then write it to UBJSON
  serde_ubjson::to_writer(&w, &model)?;
}
```

## book
Coming soon.

## crate
Coming soon.

## build
To build this project, you must have [rustup](https://rustup.rs/) preinstalled, and run our preliminary setup script (only once):

```
./setup.sh
```

Then run our build script (which goes through all the steps to build the project), or the usual cargo commands:

```sh
./build.sh
```