#![feature(io_slice_advance)]
pub mod write_all;
pub use write_all::write_all;
pub mod writev;
pub use writev::writev;
pub mod write_buffer;
pub use write_buffer::write_buffer;
