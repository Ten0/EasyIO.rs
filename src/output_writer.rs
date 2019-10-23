/*
  A fast and dead-simple writer for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Repository: https://github.com/AxlLind/EasyIO.rs
  License: MIT
  2019
*/
#![allow(dead_code)]
use std::io::{self, Write, Stdout, Result};
use std::fs::File;

pub struct OutputWriter<W: Write> {
  writer: W,
  buf: Vec<u8>,
}

impl OutputWriter<Stdout> {
  pub fn new() -> Self {
    Self::from_writer(io::stdout())
  }
}

impl OutputWriter<File> {
  pub fn from_file(path: &str) -> Self {
    let file = File::open(path).unwrap();
    Self::from_writer(file)
  }
}

impl<W: Write> OutputWriter<W> {
  pub fn from_writer(writer: W) -> Self {
    let buf = Vec::with_capacity(1 << 16);
    Self { writer, buf }
  }

  pub fn print(&mut self, s: &str) {
    self.buf.extend(s.as_bytes());
  }

  pub fn println(&mut self, s: &str) {
    self.buf.extend(s.as_bytes());
    self.buf.push(b'\n');
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
  fn drop(&mut self) { self.flush().unwrap(); }
}
