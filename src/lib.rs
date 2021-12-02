pub mod input_reader;
pub mod output_writer;

pub use input_reader::InputReader;
pub use output_writer::OutputWriter;

pub fn stdout_panics() {
	std::panic::set_hook(Box::new(|panic_info| {
		println!("{}", panic_info);
		std::process::exit(102);
	}));
}

pub mod prelude {
	pub use super::{InputReader, OutputWriter};
	pub use std::io::{Read, Write};
}
