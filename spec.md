# Kato Lang Spec

## Implementation details

### Inline vs Pointers

FIf an object's type has a known size when the program is compiled, (such as `i32`, `bool`) the value is stored directly in the object (inline). If the size is dynamic (such as a non-fixed Array, or a `String`), the object which points to the memory address of where the data is stored.

```rust
// Currently, type inference is not supported
let val1 = 10 as i32; // OK, recommended
let val2: i64 = 10;

type Person {
    // types are necessary for fields
    field name: String;
    field name default "steve";

    field age: i32;
    field age default 23 as i32;

    const species = "human";

    func greet: Void
}
```
