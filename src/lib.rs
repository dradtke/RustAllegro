#[link(name = "allegro5",
       vers = "5.0.10.1",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustAllegro")];

#[comment = "Allegro 5 core library Rust bindings"];
#[license = "zlib"];
#[crate_type = "lib"];

pub use run::*;
pub use bitmap::*;
pub use color::*;
pub use display::*;
pub use core_drawing::*;
pub use core_functions::*;

pub mod ffi;
#[macro_escape]
pub mod rust_util;

pub mod run;
pub mod bitmap;
pub mod color;
pub mod display;
pub mod core_drawing;
pub mod core_functions;