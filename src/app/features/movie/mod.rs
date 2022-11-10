mod create_handler;
mod delete_handler;
mod list_handler;
mod single_handler;
pub use create_handler::handler as create;
pub use delete_handler::handler as delete;
pub use list_handler::handler as list;
pub use single_handler::handler as movie;
pub mod entity;
