/*
  A fast and dead-simple writer for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Repository: https://github.com/AxlLind/EasyIO.rs
  License: MIT
  2019
*/

use std::{
	fmt::Display,
	fs::File,
	io::{self, Result, StdoutLock, Write},
};

pub struct OutputWriter<W: Write> {
	writer: W,
	buf: Vec<u8>,
}

impl OutputWriter<StdoutLock<'static>> {
	pub fn new() -> Self {
		Self::from_writer(io::stdout().lock())
	}
}

impl OutputWriter<File> {
	pub fn from_file(path: impl AsRef<std::path::Path>) -> Self {
		Self::from_writer(File::create(path).unwrap())
	}

	/// create parent folder if not exists
	pub fn from_file_cf(path: impl AsRef<std::path::Path>) -> Self {
		let path = path.as_ref();
		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
		Self::from_writer(File::create(path).unwrap())
	}
}

impl<W: Write> OutputWriter<W> {
	pub fn from_writer(writer: W) -> Self {
		let buf = Vec::with_capacity(1 << 16);
		Self { writer, buf }
	}

	pub fn print<T: Display>(&mut self, t: T) {
		write!(self, "{}", t).unwrap();
	}

	pub fn prints<T: Display>(&mut self, t: T) {
		write!(self, "{} ", t).unwrap();
	}

	pub fn println<T: Display>(&mut self, t: T) {
		writeln!(self, "{}", t).unwrap();
	}

	pub fn s2nl(&mut self) {
		match self.buf.last_mut() {
			Some(last) => match *last {
				b' ' => *last = b'\n',
				b'\n' => {}
				_ => self.buf.push(b'\n'),
			},
			None => panic!("Buffer is empty"),
		}
	}

	pub fn nl(&mut self) {
		self.buf.push(b'\n');
	}

	pub fn yesno(&mut self, b: bool) {
		self.println(if b { "YES" } else { "NO" })
	}
}

impl<W: Write> Write for OutputWriter<W> {
	fn write(&mut self, bytes: &[u8]) -> Result<usize> {
		self.buf.extend(bytes);
		Ok(bytes.len())
	}

	fn flush(&mut self) -> Result<()> {
		self.writer.write_all(&self.buf)?;
		self.writer.flush()?;
		self.buf.clear();
		Ok(())
	}
}

impl<W: Write> Drop for OutputWriter<W> {
	fn drop(&mut self) {
		if !self.buf.is_empty() {
			self.s2nl();
		}
		self.flush().unwrap();
	}
}
