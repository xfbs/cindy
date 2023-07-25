use crate::config::HashAlgorithm;
pub use cindy_common::{ArcHash, BoxHash, Hash};
use digest::DynDigest;
use std::io::{Read, Result as IoResult};

pub trait Digester: std::fmt::Debug {
    /// Create new Hasher
    fn create(&self) -> Box<dyn DynDigest + Send>;

    /// Determine Hash output size.
    fn output_size(&self) -> usize {
        self.create().output_size()
    }
}

impl Digester for HashAlgorithm {
    fn create(&self) -> Box<dyn DynDigest + Send> {
        use HashAlgorithm::*;
        match self {
            Blake2b512 => Box::<blake2::Blake2b512>::default() as _,
            Blake2s256 => Box::<blake2::Blake2s256>::default() as _,
        }
    }
}

/// Trait to hash data.
pub trait DataHasher {
    fn hash_data(&self, data: &[u8]) -> BoxHash;
}

impl<T: Digester + ?Sized> DataHasher for T {
    fn hash_data(&self, data: &[u8]) -> BoxHash {
        let mut hasher = self.create();
        hasher.update(data);
        hasher.finalize().into()
    }
}

const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

pub trait ReadDigester {
    fn hash_read(&self, reader: &mut dyn Read) -> IoResult<BoxHash> {
        self.hash_read_bufsize(reader, DEFAULT_BUFFER_SIZE)
    }

    fn hash_read_bufsize(&self, reader: &mut dyn Read, bufsize: usize) -> IoResult<BoxHash>;
}

impl<T: Digester + ?Sized> ReadDigester for T {
    fn hash_read_bufsize(&self, reader: &mut dyn Read, buffer_size: usize) -> IoResult<BoxHash> {
        let mut hasher = self.create();
        let mut buffer = vec![0; buffer_size];
        loop {
            let read = reader.read(&mut buffer[..])?;
            if read == 0 {
                return Ok(hasher.finalize().into());
            }
            hasher.update(&buffer[0..read]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test vector that is longer than the default buffer size
    const LONG_DATA: &[u8] = &[17; 5 * DEFAULT_BUFFER_SIZE / 2];

    const BLAKE2B512_HASH_EMPTY: [u8; 64] = [
        120, 106, 2, 247, 66, 1, 89, 3, 198, 198, 253, 133, 37, 82, 210, 114, 145, 47, 71, 64, 225,
        88, 71, 97, 138, 134, 226, 23, 247, 31, 84, 25, 210, 94, 16, 49, 175, 238, 88, 83, 19, 137,
        100, 68, 147, 78, 176, 75, 144, 58, 104, 91, 20, 72, 183, 85, 213, 111, 112, 26, 254, 155,
        226, 206,
    ];

    const BLAKE2B512_HASH_HELLO: [u8; 64] = [
        228, 207, 163, 154, 61, 55, 190, 49, 197, 150, 9, 232, 7, 151, 7, 153, 202, 166, 138, 25,
        191, 170, 21, 19, 95, 22, 80, 133, 224, 29, 65, 166, 91, 161, 225, 177, 70, 174, 182, 189,
        0, 146, 180, 158, 172, 33, 76, 16, 60, 207, 163, 163, 101, 149, 75, 187, 229, 47, 116, 162,
        179, 98, 12, 148,
    ];

    const BLAKE2B512_HASH_WORLD: [u8; 64] = [
        38, 123, 46, 2, 182, 175, 61, 91, 255, 82, 57, 127, 35, 140, 58, 36, 14, 93, 19, 25, 55,
        93, 197, 198, 14, 249, 240, 223, 90, 233, 81, 23, 97, 150, 143, 51, 162, 206, 23, 160, 149,
        45, 133, 254, 210, 49, 230, 16, 60, 134, 127, 4, 50, 194, 80, 221, 82, 186, 249, 89, 199,
        252, 71, 89,
    ];

    #[test]
    fn can_hash_data_blake2b() {
        let algorithm = HashAlgorithm::Blake2b512;
        assert_eq!(algorithm.hash_data(b""), BLAKE2B512_HASH_EMPTY);
        assert_eq!(algorithm.hash_data(b"hello"), BLAKE2B512_HASH_HELLO);
        assert_eq!(algorithm.hash_data(b"world"), BLAKE2B512_HASH_WORLD);
    }

    #[test]
    fn can_hash_read_blake2b() {
        let algorithm = HashAlgorithm::Blake2b512;
        assert_eq!(
            algorithm.hash_read(&mut &b""[..]).unwrap(),
            BLAKE2B512_HASH_EMPTY
        );
        assert_eq!(
            algorithm.hash_read(&mut &b"hello"[..]).unwrap(),
            BLAKE2B512_HASH_HELLO
        );
        assert_eq!(
            algorithm.hash_read(&mut &b"world"[..]).unwrap(),
            BLAKE2B512_HASH_WORLD
        );
    }

    const BLAKE2S256_HASH_EMPTY: [u8; 32] = [
        105, 33, 122, 48, 121, 144, 128, 148, 225, 17, 33, 208, 66, 53, 74, 124, 31, 85, 182, 72,
        44, 161, 165, 30, 27, 37, 13, 253, 30, 208, 238, 249,
    ];

    const BLAKE2S256_HASH_HELLO: [u8; 32] = [
        25, 33, 59, 172, 197, 141, 238, 109, 189, 227, 206, 185, 164, 124, 187, 51, 11, 61, 134,
        248, 204, 168, 153, 126, 176, 11, 228, 86, 241, 64, 202, 37,
    ];

    const BLAKE2S256_HASH_WORLD: [u8; 32] = [
        127, 150, 209, 144, 232, 9, 183, 35, 140, 129, 86, 240, 30, 42, 128, 83, 137, 184, 155, 88,
        73, 48, 7, 253, 182, 177, 185, 244, 179, 247, 23, 153,
    ];

    #[test]
    fn can_hash_data_blake2s() {
        let algorithm = HashAlgorithm::Blake2s256;
        assert_eq!(algorithm.hash_data(b""), BLAKE2S256_HASH_EMPTY);
        assert_eq!(algorithm.hash_data(b"hello"), BLAKE2S256_HASH_HELLO);
        assert_eq!(algorithm.hash_data(b"world"), BLAKE2S256_HASH_WORLD);
    }

    #[test]
    fn can_hash_read_blake2s() {
        let algorithm = HashAlgorithm::Blake2s256;
        assert_eq!(
            algorithm.hash_read(&mut &b""[..]).unwrap(),
            BLAKE2S256_HASH_EMPTY
        );
        assert_eq!(
            algorithm.hash_read(&mut &b"hello"[..]).unwrap(),
            BLAKE2S256_HASH_HELLO
        );
        assert_eq!(
            algorithm.hash_read(&mut &b"world"[..]).unwrap(),
            BLAKE2S256_HASH_WORLD
        );
    }

    const BLAKE2B512_HASH_LONG: [u8; 32] = [
        18, 214, 217, 86, 171, 0, 180, 29, 148, 114, 255, 53, 23, 224, 195, 229, 220, 224, 216, 2,
        180, 26, 53, 191, 14, 248, 23, 160, 113, 224, 84, 21,
    ];

    #[test]
    fn can_hash_read() {
        let algorithm = HashAlgorithm::Blake2s256;
        assert_eq!(
            algorithm.hash_read(&mut &LONG_DATA[..]).unwrap(),
            BLAKE2B512_HASH_LONG
        );
        for bufsize in [1, 2, 5, 12, 33] {
            assert_eq!(
                algorithm
                    .hash_read_bufsize(&mut &LONG_DATA[..], bufsize)
                    .unwrap(),
                BLAKE2B512_HASH_LONG
            );
        }
    }

    #[test]
    fn blake2b_output_size() {
        let algorithm = HashAlgorithm::Blake2b512;
        assert_eq!(algorithm.output_size(), 64);
    }

    #[test]
    fn blake2s_output_size() {
        let algorithm = HashAlgorithm::Blake2s256;
        assert_eq!(algorithm.output_size(), 32);
    }
}
