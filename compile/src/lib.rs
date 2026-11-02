// lib.rs
// takes a typed AST from core/src/typecheck and uses cranelift
// to turn it into the clif IR, which is then compiled
// into ASM. so I never actually have to touch assembley, hooray.

// fn compile(tree)
