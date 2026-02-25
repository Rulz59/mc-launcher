/// Prints a personalized greeting message to the console.
///
/// # Parameters
/// - `name`: The name of the person or entity to greet.
/// - `times`: The number of times the greeting should be printed.
///
/// # Example
/// ```
/// use rust_template::module_lib::hello_world::greet;
///
/// greet("John", 3);
/// // Output:
/// // Hello, John!
/// // Hello, John!
/// // Hello, John!
/// ```
///
/// # Panics (Explain under what conditions this function will panic)
/// This function will panic if `times` is set to 0.
///
/// # Errors (Describe any errors that may be returned)
/// None.
///
/// # Safety (Document any safety considerations)
/// This function is safe to use and does not perform any unsafe operations.
pub fn greet(name: &str, times: usize) {
    if times == 0 {
        panic!("The `times` parameter must be greater than 0.");
    }

    for _ in 0..times {
        println!("Hello, {}!", name);
    }
}

 
#[doc(hidden)] // Hide the function from the documentation
pub fn goodbye() {
    println!("Goodbye!");
}