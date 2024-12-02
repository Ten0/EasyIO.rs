/*
  A fast and dead-simple reader for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Repository: https://github.com/AxlLind/EasyIO.rs
  License: MIT
  2019
*/

use std::{
	fs::File,
	io::{self, Read, StdinLock},
};

pub struct InputReader<R: Read> {
	reader: R,
	buf: Vec<u8>,
	bytes_read: usize,
	current_index: usize,
	str_buf: String,
	/// This is typically good in cases where entire input is provided
	/// (it makes `has_more` accurate)
	///
	/// However it shouldn't be used if input depends on your output
	/// as this may result in a deadlock (e.g. CodinGame)
	egearly_consume_whitespace: bool,
}

impl InputReader<StdinLock<'static>> {
	pub fn new() -> Self {
		Self::from_reader(io::stdin().lock(), true)
	}
	/// This is useful if playing games like CodinGame where input is not provided
	/// all at once, but instead depends on your output
	///
	/// Eager whitespace consumption there would result in deadlock
	pub fn without_eager_whitespace_consumption() -> Self {
		Self::from_reader(io::stdin().lock(), false)
	}
}

impl InputReader<File> {
	pub fn from_file(path: impl AsRef<std::path::Path>) -> Self {
		Self::from_reader(File::open(path).unwrap(), true)
	}
}

impl<R: Read> InputReader<R> {
	pub fn from_reader(reader: R, egearly_consume_whitespace: bool) -> Self {
		let mut ir = Self {
			reader,
			buf: vec![0; 1 << 16],
			bytes_read: 0,
			current_index: 0,
			str_buf: String::with_capacity(1 << 8),
			egearly_consume_whitespace,
		};
		if egearly_consume_whitespace {
			ir.consume_until_or_end(|c| c.is_ascii_graphic());
		}
		ir
	}

	pub fn next_word(&mut self) -> &str {
		if !self.egearly_consume_whitespace {
			self.consume_until(|c| c.is_ascii_graphic());
		}

		self.str_buf.clear();
		while self.peek().is_ascii_graphic() {
			let c = self.peek();
			self.str_buf.push(c);
			self.consume();
			if !self.has_more() {
				break;
			}
		}
		if self.egearly_consume_whitespace {
			self.consume_until_or_end(|c| c.is_ascii_graphic());
		}
		&self.str_buf
	}

	pub fn next_line(&mut self) -> &str {
		self.next_line_no_skip();
		if self.egearly_consume_whitespace {
			self.consume_until_or_end(|c| c.is_ascii_graphic());
		}
		&self.str_buf
	}

	pub fn next_line_no_skip(&mut self) -> &str {
		self.str_buf.clear();
		loop {
			let c = self.peek();
			self.consume();
			match c {
				'\n' => break,
				other => self.str_buf.push(other),
			}
			if !self.has_more() {
				break;
			}
		}
		&self.str_buf
	}

	pub fn next_char(&mut self) -> char {
		if !self.egearly_consume_whitespace {
			self.consume_until(|c| c.is_ascii_graphic());
		}
		let c = self.peek();
		self.consume();
		if self.egearly_consume_whitespace {
			self.consume_until_or_end(|c| c.is_ascii_graphic());
		}
		c
	}

	pub fn next_u64(&mut self) -> u64 {
		self.consume_until(|c| c.is_ascii_digit());
		let mut num = 0;
		while self.peek().is_ascii_digit() {
			let digit = self.peek() as u64 - '0' as u64;
			num = num * 10 + digit;
			self.consume();
			if !self.has_more() {
				break;
			}
		}

		if self.egearly_consume_whitespace {
			self.consume_until_or_end(|c| c.is_ascii_graphic());
		}
		num
	}

	pub fn next_i64(&mut self) -> i64 {
		let sign = self.consume_until_signed_num();
		self.next_u64() as i64 * sign
	}

	pub fn next_f64(&mut self) -> f64 {
		let sign = self.consume_until_signed_num() as f64;
		let num: f64 = self.next_word().parse().unwrap();
		num * sign
	}

	pub fn has_more(&mut self) -> bool {
		if self.current_index >= self.bytes_read {
			self.bytes_read = self.reader.read(&mut self.buf[..]).unwrap();
			self.current_index = 0
		}
		self.bytes_read > 0
	}

	pub fn set_buf_size(&mut self, buf_size: usize) {
		assert!(
			buf_size >= self.bytes_read,
			"InputReader: Data loss while shrinking buffer!"
		);
		self.buf.resize(buf_size, 0);
	}

	pub fn next_f32(&mut self) -> f32 {
		self.next_f64() as f32
	}
	pub fn next_i8(&mut self) -> i8 {
		self.next_i64() as i8
	}
	pub fn next_i16(&mut self) -> i16 {
		self.next_i64() as i16
	}
	pub fn next_i32(&mut self) -> i32 {
		self.next_i64() as i32
	}
	pub fn next_u8(&mut self) -> u8 {
		self.next_u64() as u8
	}
	pub fn next_u16(&mut self) -> u16 {
		self.next_u64() as u16
	}
	pub fn next_u32(&mut self) -> u32 {
		self.next_u64() as u32
	}
	pub fn next_usize(&mut self) -> usize {
		self.next_u64() as usize
	}

	pub fn peek(&mut self) -> char {
		self.assert_has_more();
		self.buf[self.current_index] as char
	}

	pub fn opt_peek(&mut self) -> Option<char> {
		if self.has_more() {
			Some(self.buf[self.current_index] as char)
		} else {
			None
		}
	}
}

// private methods
impl<R: Read> InputReader<R> {
	fn consume(&mut self) {
		self.current_index += 1;
	}

	fn assert_has_more(&mut self) {
		assert!(self.has_more(), "InputReader: Reached end of input!");
	}

	fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
		while !test(self.peek()) {
			self.consume();
		}
	}

	fn consume_until_or_end<F: Fn(char) -> bool>(&mut self, test: F) {
		while self.opt_peek().map_or(false, |c| !test(c)) {
			self.consume();
		}
	}

	fn consume_until_signed_num(&mut self) -> i64 {
		loop {
			self.consume_until(|c| c.is_ascii_digit() || c == '-');
			if self.peek() != '-' {
				return 1;
			}

			self.consume();
			if self.peek().is_ascii_digit() {
				return -1;
			}
		}
	}
}
