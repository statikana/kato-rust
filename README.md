# kato-rust

1. `parse`: Compiles `Kato.g4` into a node visitor using antlr4rust
2. `ast`: Creates `ASTVisitor`, exhausts the tree and creates a base untyped AST with `ASTNode`
3. `typed`: Travels across the untyped AST and applies type TypeAlgebra rules to each node recursively, generated `TypedNode`
4. `clif`: Travels the typed AST and and transforms it into clif using cranelift
5. `compile`: Compiles the generated clif code into an executable binary

`run` serves as an entrypoint to the program, taking an input file and sending it through the pipeline.

Current progress:

- [X] `parse`
- [X] `ast`
- [X] `typed`
- [ ] `clif`
- [ ] `compile`

Next step: Begin transcribing operators and functions from the language into functions which return instructions. Then, make a Typed AST walker which reads the tree and uses the functions to create a .clif
