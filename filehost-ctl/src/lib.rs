/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:32:50
 * Last edited:
 *   16 Apr 2022, 16:41:06
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the library part of the FileHost CTL-binary.
**/

// Use the macros from some external crates
#[macro_use] extern crate lazy_static;

/// Module that collects the errors in the crate.
pub mod errors;
/// Module that handles the Command-Line Interface parsing.
pub mod cli;
