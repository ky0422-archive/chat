pub mod handler;

pub use handler::*;

pub fn handle_error<T, E>(result: Result<T, E>)
where
    E: std::error::Error,
{
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
