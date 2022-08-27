use std::fs;
use std::fs::File;
use std::io::Read;

/// Allows for easy reading of the raw bytes of a file.
pub struct Reader {
    pub bytes: Vec<u8>,
    pub index: usize,
}

/// Creates a reader for a given file.
pub fn new(filename: &str) -> Reader {
    let filename_string = filename.to_string();
    let mut f = File::open(&filename_string).expect("no file found");
    let metadata = fs::metadata(&filename_string).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    Reader {
        bytes: buffer,
        index: 0,
    }
}

impl Reader {
    /// Reads and advances a single byte.
    pub fn g1(&mut self) -> u8 {
        self.index += 1;
        self.bytes[self.index - 1]
    }

    /// Reads and advances two bytes.
    pub fn g2(&mut self) -> u16 {
        (self.g1() as u16) << 8 | (self.g1() as u16)
    }

    /// Reads and advances four bytes.
    pub fn g4(&mut self) -> u32 {
        (self.g1() as u32) << 24
            | (self.g1() as u32) << 16
            | (self.g1() as u32) << 8
            | (self.g1() as u32)
    }

    /// Reads and advances eight bytes.
    pub fn g8(&mut self) -> u64 {
        (self.g1() as u64) << 56
            | (self.g1() as u64) << 48
            | (self.g1() as u64) << 40
            | (self.g1() as u64) << 32
            | (self.g1() as u64) << 24
            | (self.g1() as u64) << 16
            | (self.g1() as u64) << 8
            | (self.g1() as u64)
    }

    /// Reads and advances a passed number of bytes.
    pub fn g(&mut self, size: usize) -> Vec<u8> {
        self.index += size;
        self.bytes[self.index - size..self.index].to_vec()
    }

    /// Read the current index.
    pub fn pos(&self) -> usize {
        self.index
    }

    /// Set the current index to a given value.
    pub fn set_pos(&mut self, pos: usize) {
        self.index = pos;
    }
}