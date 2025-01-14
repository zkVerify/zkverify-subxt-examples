# zkverify-subxt-examples

First, generate an interface that we can use by retrieving the runtime's metadata from a node:

```
cargo install subxt-cli
subxt metadata --url wss://testnet-rpc.zkverify.io > zk_verify_runtime_metadata.scale
```

You can find a list of examples you can run in the ```Cargo.toml``` file:

```
[[example]]
name = "submit_proof_basic"
path = "examples/submit_proof_basic.rs"
```

You can run them by executing the following command:

```
cargo run --example example_name
```
