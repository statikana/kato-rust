# Kato-Rust

Running:

1. Install the deps:

    - rustc 1.100-nightly (nightly needed for Kato type system, i.e. `core/src/datatype.rs`)
    - antlr-rust 0.3.0-beta (library needed for both generated runtime and core execution)
    - antlr4rust runtime generator (JAR file) ([github link](https://github.com/rrevenantt/antlr4rust), it should already be in antlr4/ though)

2. Generate runtime: run `./comp` or
    - Run the antlr4-complete jar: `java -jar antlr4/antlr-X-X-XXX-SNAPSHOT-complete.jar Kato.g4 -o generated/src -Dlanguage=Rust -visitor`
    - Fix managable errors in generated code: `cargo fix --lib --allow-dirty` (make sure you've got cargo on nightly rust like [this](https://doc.rust-lang.org/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html))
    - `cargo build`
    - `cargo run` to run the executor on, by default, `input.txt`. This can just be changed in `core/src/main.rs` 
