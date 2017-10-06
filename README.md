# slurp

some little helper functions to load files without the boilerplate [![Build Status](https://travis-ci.org/QuietMisdreavus/slurp.svg?branch=master)](https://travis-ci.org/QuietMisdreavus/slurp) [![Build status](https://ci.appveyor.com/api/projects/status/rf0xehtwonwvw0qy/branch/master?svg=true)](https://ci.appveyor.com/project/QuietMisdreavus/slurp/branch/master)

`slurp` exists to solve a very particular problem. If you're trying to load a file from disk,
there's some boilerplate code that you need to write up to handle the `Read` trait effectively:

```rust
// gotta declare the output before??
let mut buf = String::new();
// gotta declare the file reader separately??
let mut file = File::open(filename).unwrap();

file.read_to_string(&mut buf).unwrap();
```

Now, there's a really good reason `Read` doesn't allocate its own Vecs or Strings: This way, you can
really easily reuse your own buffers. But what if you want to hang onto each file's contents
separately? Or what if you're only loading one file ever? All this extra ability just gets in the
way when you just want to `slurp` a file off the disk.

That's where this crate comes in! By adding this crate (or copying its functions into your project -
they're really quite small), all that boilerplate gets wrapped away into its own function!

```rust
let my_file = slurp::read_all_to_string(filename).unwrap();
```

`slurp` also contains other functions to load into a byte vector (`read_all_bytes`) or into a Vec
with one String for each line (`read_all_lines`).

To use this crate in your project, add the following to your Cargo.toml:

```toml
[dependencies]
slurp = "1.0.0"
```

...and the following in your crate root:

```rust
extern crate slurp;
```

## License

`slurp` is licensed under the Unlicense, which is effectively a public domain dedication. If you
don't want to add an entirely separate crate for this, feel free to copy the code into your own
project!
