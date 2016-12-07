# Rust Notes

## Packt: Rust Essentials

### Overview of Pointers

* `&T` (Reference) - This allows one or more references to read `T`.
* `&mut T` (Mutable Reference) - This allows a single reference to read and write `T`.
* `Box<T>` (Box) - This is a heap-allocated `T` with a single owner that may read and write `T`.
* `Rc<T>` (Rc Pointer) - This is a heap-allocated `T` with many readers.
* `Arc<T>` (Arc Pointer) - This is like `Rc<T>`, but enables safe mutable sharing accress threads.
* `*const T` (Raw Pointer) - This allows unsafe read access to `T`.
* `*mut T` (Mutable Raw Pointer) - This allows unsafe read and write access to `T`.

### Going Deeper

* Chapter 7. Organizing Code and Macros
* Chapter 8. Concurrency and Parallelism

### Developer Tools

#### Sublime Text

* Rust Enhanced
* SublimeLinter-contrib-rustc
* RustAutoComplete
