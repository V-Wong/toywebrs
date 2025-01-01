pub mod prelude {
    pub use crate::http::request::{Method, Request};
    pub use crate::http::response::{Response, Status};
    pub use crate::router::Router;
    pub use crate::thread_pool::ThreadPool;
}

mod http;
mod router;
mod thread_pool;
