/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:32:50
 * Last edited:
 *   11 Jun 2022, 13:30:37
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the library part of the FileHost server.
**/

/// Module that collects the errors in the crate.
pub mod errors;
/// Modules that does the complicated SSL junk.
pub mod ssl;
/// Modules that interacts with some user database.
pub mod users;
