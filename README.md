

# Ollama wrapper
cli tool for using llama3 with custom instruction prompt

# To install

1. First install ollama https://ollama.com/download
1. ollama run llama3
 
```sh
git clone https://github.com/ax-sh/ahh.git
cargo install --path .

# OR

cargo install --git https://github.com/ax-sh/ahh.git
```


# Examples
```sh

cargo run

cargo run -- --help

cargo run -- this is a test
echo "using piped data" | cargo run -- this is a test

OR

echo "using piped data" | ahh this is a test


```

## Ref
- https://www.shuttle.rs/blog/2023/12/08/clap-rust
- https://www.rustadventure.dev/building-a-digital-garden-cli/clap-v4/implementing-subcommands-with-clap#:~:text=The%20we%20use%20the%20command,as%20the%20flags%20they%20take.

- https://github.com/ad-si/cai/blob/main/tests/integration_tests.rs

- https://crates.io/crates/clap-stdin