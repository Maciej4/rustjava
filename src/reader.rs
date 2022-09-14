//! A utility for reading a file byte by byte.
use std::fs;
use std::fs::File;
use std::io::Read;

/// Allows for the easy reading of the raw bytes of a file in an incremental way.
pub struct Reader {
    pub bytes: Vec<u8>,
    pub index: usize,
}

impl Reader {
    /// Make a new reader for a passed file.
    pub fn new(filename: String) -> Reader {
        let filename_string = filename;
        let mut f = File::open(&filename_string).expect("no file found");
        let metadata = fs::metadata(&filename_string).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer).expect("buffer overflow");

        Reader {
            bytes: buffer,
            index: 0,
        }
    }

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

    /// Reads and advances a passed number of bytes.
    pub fn g(&mut self, size: usize) -> Vec<u8> {
        self.index += size;
        self.bytes[self.index - size..self.index].to_vec()
    }

    /// Read and advance 4 bytes and return a four length array of u8.
    pub fn g4_array(&mut self) -> [u8; 4] {
        let mut array = [0; 4];
        array.copy_from_slice(&self.g(4));
        array
    }

    /// Read and advance 8 bytes and return an eight length array of u8.
    pub fn g8_array(&mut self) -> [u8; 8] {
        let mut array = [0; 8];
        array.copy_from_slice(&self.g(8));
        array
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
