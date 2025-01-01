pub mod prelude {
    pub use crate::request::{Method, Request};
    pub use crate::response::{Response, Status};
    pub use crate::router::Router;
    pub use crate::thread_pool::ThreadPool;
}

mod request;
mod response;
mod router;
mod thread_pool;
