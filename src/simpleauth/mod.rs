extern crate rocket;
pub mod authenticator;

pub mod status;
pub mod userpass;

mod config;
/// Example implementation of Authenticator and FromCookie and ToCookie
pub mod dummy;
