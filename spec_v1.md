# kato lang spec

## Objects

### Syntax

- `def ID: T`
Allocates space in memory of size size(T) and points ID at the start of it.

- `set ID = DATA`
Writes DATA in memory starting from where ID points. DATA must be of type :ID:

- `let ID: T = DATA`
Shorthand for:

  ```rust
  def ID: T;
  set ID = DATA;
  ```

- `:ID:`
Gets the type of ID. Not sure exactly how this will be implemented

- `attr`
Used when defining an attribute of a type

#### `static` and `dynamic` objects

- `static` objects
  - Set size at compile time (like pointers, `int32`, `float128`, `matrix<int>[2, 1]`), they use the stack

  - Annotated with snake case spelling

  - Lives directly on the stack, so `stdout.write(16 as i32)` outputs `"16"`

- `dynamic` objects
  - Unknown size at compile time, so they create space on the heap at runtime (like `Array`, `Stirng`)
  - Annotated with upper camel case spelling
  - They are handled by pointers (which do live in the stack), so `stdout.write(my_array)` outputs the memory address to which the object's stack pointer points to

For now, each variable must be annotated with its type and whether it is `static` or `dynamic` for clarity.

#### Defining an object

Two categories of annotations in types

- Constant value, annotated with `[]`
  - Always `constant`
  - In a `static` class, only `static` values are permitted
- Type, annotated with `<>`
  - Can be any type (`static` or `dynamic`), since the the type itself is a constant size (even if an object of that type may be `dynamic`, and therefore have a variable size)
  - Basically a generic

#### Pointers

Technically everything is a pointer. the only difference is that a static object is a pointer whose value in in the stack, and thus it is only mostly used by the compiler. A dynamic object's value is the memory address to where its value is stored.

### Defining an type

The typing of a class name (upper camel and snake) is just convention, the static and dynamic tags are used to enforce the object type.

#### Constant static type parameters

The constant parameter's, typed with `[ID: TYPE]` names are how they're used internally by the object

#### Dynamic type parameters

The type parameters are typed using `<TYPE, ...>`

### Examples

```rust
def static favorite_number: i32;
set favorite_number = 100;

def dynamic name: String;
set name = "Black Knight";
set name = "The " + name; // produces

set goal = "The Holy Grail";
// Error: `goal` is not defined; use `let` instead to define and assign.

let const dynamic slayer: String = "King Arthur";

// for now, definitions are extremely verbose. Ideally, if I make a strong enough type system you could just
let phrase = "Hello"; // <- not valid (yet)
// the `dynamic` tag and `String` type annotation are both inferred. 

// Builtin static type: matrix
def static state: matrix<i32>[2, 1];
// matirx uses variable value parameters as the shape of the array
// doing this allows the shape and therefore size to be known at compile-time, thus
// making the matrix type static.

set state = [100, 12 : i32].like(:state:); // :state: is `matrix<i32>[2, 1]`, not just `matrix`
// the syntax [x1, x2, x3, ..., xn : T] makes matrix[n]<T>
// the .like creates a new array in the shape of the given array

// the above code can be simplifed into
let static state: matrix<i32>[2, 1] = [100, 12 : i32].shape_to([2, 1])

// Accessing elements:
set state[0, 0] = 10 as i32;
stdout.write(state[0, 0]);
// number of elements in index must match the number of dimensions
// in the array, i.e. len(index) == len(shape)
// I love numpy but you cannot do state[0] and get the first row
// or state[:, 0] and get the first value of each row etc. like in numpy

// item indexing and assignment are controlled by special methods in the implementation of the matrix type

// methods: like, shaped, ...?
```

quick note: `static` does not mean `const`. `const` means the value of the object does not change during runtime, and it used to protect the intended use of a variable. `dynamic` values may be `const`. `static` means that the size is known at compile time. this value may change (i.e. the size of `3 as i32` and `100 as i32` are both of size `sizeof(i32)`). the `static` keyword is used by the compiler, mostly.
