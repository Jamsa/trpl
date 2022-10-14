// mod macros;

/*
 * macro vs function:
 *  - Macros expand at compile time, functions are called at runtime.
 *  - Macro definitions are difficult to read and maintain
 */

/*
 * macro category
 * - declarative macro
 * - procedure macro
 *   - derive
 *   - attribute
 *   - function like
 */

// A simple version of the macro vec!
// a declarative macro
#[macro_export]
macro_rules! vecx {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
        temp_vec
    }
  };
}
