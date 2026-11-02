# kato-rust

1. `parse`: Compiles `Kato.g4` into a node visitor using antlr4rust
2. `ast`: Creates `ASTVisitor`, exhausts the tree and creates a base, untyped AST
3. `typed`: Travels across the untyped AST and applies type TypeAlgebra rules to each type recursively
4. `compile`: 