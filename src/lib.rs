pub mod prelude {
    pub use crate::request::{Method, Request};
    pub use crate::response::{Response, Status};
    pub use crate::router::Router;
    pub use crate::thread_pool::ThreadPool;
}

pub mod request;
pub mod response;
pub mod router;
pub mod thread_pool;
