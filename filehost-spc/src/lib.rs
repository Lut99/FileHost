/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   30 Mar 2022, 19:36:09
 * Last edited:
 *   11 Jun 2022, 11:29:08
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the library that contains the (communication)
 *   specification of the filehost project.
**/

/// Module that contains the configuration file definition.
pub mod config;
/// Module that contains messages and structs for logging in to the server.
pub mod login;
/// Module that contains messages between the CTL and the daemon.
pub mod ctl_messages;
