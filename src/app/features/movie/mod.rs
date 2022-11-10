mod create_handler;
pub mod errors;
mod list_handler;
mod single_handler;
pub use create_handler::handler as create;
pub use list_handler::handler as list;
pub use single_handler::handler as movie;
pub mod entity;
