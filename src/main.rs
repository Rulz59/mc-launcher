/// 'rust_template' - A Rust template project with a nested library module.
/// The library module imports all the modules in the `src` directory.
/// The library module is also capable of allowing other applications to use it as a library.

use rust_template::module_lib;

/// Main function. Runs the greet function from the hello_world which is part of the module_lib crate.
/// Alternatively, you can use the full path when importing the module.
/// 
/// # Example usage:
/// ```Import:```
/// use rust_template::module_lib::hello_world::greet;
/// 
/// ``` Main function: ```
/// fn main() {
///    greet();
/// }
/// 
fn main() {
    module_lib::hello_world::greet("John", 3);

}