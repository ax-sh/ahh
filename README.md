
```sh

cargo run

cargo run -- --help

cargo run -- this is a test
echo "using piped data" | cargo run -- this is a test


```

```sh

# to install
git clone https://github.com/ax-sh/ahh.git
cargo install --path .
```

## Ref
- https://www.shuttle.rs/blog/2023/12/08/clap-rust
- https://www.rustadventure.dev/building-a-digital-garden-cli/clap-v4/implementing-subcommands-with-clap#:~:text=The%20we%20use%20the%20command,as%20the%20flags%20they%20take.