# sync-code

Synchronize code blocks between different files.

It can replace `macros` in certain scenarios. For example, when your code isn’t duplicated many times but the design is relatively complex (e.g., involving generics), using `sync-code` instead of a `macro` will give you much better readability and easier maintenance. Features like Go-to-definition and intelligent auto-completion also work properly without using `macros`.

# Usage
`Cargo.toml`:
```shell
cargo add --build sync-code
```

See [crate](https://crates.io/crates/sync-code).

`build.rs`:
```rust
fn main() {
    sync_code::Builder::new()
        .add("src/target1.rs", "src/source1.rs")
        .add("src/target2.rs", "src/source2.rs")
        .sync();
}

```

`your_code.rs`:
```rust
// $sync block_name

fn code_you_want_to_sync() {
}

// $sync end
```

See also [tests](/tests/test.rs)
