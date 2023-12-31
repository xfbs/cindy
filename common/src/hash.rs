use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    borrow::{Borrow, ToOwned},
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
    rc::Rc,
    str::FromStr,
    sync::Arc,
};

/// Represents a hash value with generic storage.
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord, Hash)]
pub struct Hash<T: ?Sized + AsRef<[u8]> = [u8]>(T);

impl Hash {
    pub fn new<H: AsRef<[u8]> + ?Sized>(hash: &H) -> &Self {
        unsafe { &*(hash.as_ref() as *const [u8] as *const Hash) }
    }
}

impl<T: ?Sized + AsRef<[u8]>> Hash<T> {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T: Sized + AsRef<[u8]>> Deref for Hash<T> {
    type Target = Hash;

    fn deref(&self) -> &Self::Target {
        Hash::new(&self.0)
    }
}

impl Deref for Hash {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type BoxHash = Hash<Box<[u8]>>;
pub type ArcHash = Hash<Arc<[u8]>>;
pub type RcHash = Hash<Rc<[u8]>>;
pub type RefHash<'a> = Hash<&'a [u8]>;

impl<T: AsRef<[u8]> + ?Sized> AsRef<Hash> for Hash<T> {
    fn as_ref(&self) -> &Hash {
        Hash::<[u8]>::new::<T>(&self.0)
    }
}

impl<T: AsRef<[u8]>> From<T> for Hash<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: ?Sized + AsRef<[u8]>> Display for Hash<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let slice = self.0.as_ref();
        for byte in slice {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<T: ?Sized + AsRef<[u8]>> AsRef<[u8]> for Hash<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T: AsRef<[u8]> + From<Vec<u8>>> From<&Hash> for Hash<T> {
    fn from(hash: &Hash) -> Self {
        Self(hash.0.to_vec().into())
    }
}

impl<T: AsRef<[u8]> + From<Vec<u8>>> FromStr for Hash<T> {
    type Err = hex::FromHexError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        hex::decode(input).map(Into::into).map(Self)
    }
}

impl<L: ?Sized + AsRef<[u8]>, R: ?Sized + AsRef<[u8]>> PartialEq<Hash<R>> for Hash<L> {
    fn eq(&self, other: &Hash<R>) -> bool {
        self.0.as_ref().eq(other.0.as_ref())
    }
}

impl<T: ?Sized + AsRef<[u8]>> PartialEq<[u8]> for Hash<T> {
    fn eq(&self, other: &[u8]) -> bool {
        self.0.as_ref().eq(other)
    }
}

impl<T: ?Sized + AsRef<[u8]>, const L: usize> PartialEq<[u8; L]> for Hash<T> {
    fn eq(&self, other: &[u8; L]) -> bool {
        self.0.as_ref().eq(&other[..])
    }
}

impl<T: AsRef<[u8]>> Serialize for Hash<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serialize as hex string or as byte array, depending on format.
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            self.0.as_ref().serialize(serializer)
        }
    }
}

impl<'de, T: AsRef<[u8]> + From<Vec<u8>>> Deserialize<'de> for Hash<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let data: &'de str = <&'de str>::deserialize(deserializer)?;
            Ok(Self::from_str(data).map_err(Error::custom)?)
        } else {
            let data: Vec<u8> = <Vec<u8>>::deserialize(deserializer)?;
            Ok(Self(data.into()))
        }
    }
}

impl<T: AsRef<[u8]>> Borrow<Hash> for Hash<T> {
    fn borrow(&self) -> &Hash {
        Hash::new(self.0.as_ref())
    }
}

impl ToOwned for Hash {
    type Owned = Hash<Box<[u8]>>;

    fn to_owned(&self) -> Self::Owned {
        Hash(Box::from(self.0.to_vec()))
    }
}

impl<T: AsRef<[u8]>> From<&T> for BoxHash {
    fn from(value: &T) -> Self {
        Hash(value.as_ref().to_vec().into())
    }
}

impl From<BoxHash> for ArcHash {
    fn from(hash: BoxHash) -> Self {
        Self(hash.0.into())
    }
}

impl From<BoxHash> for RcHash {
    fn from(hash: BoxHash) -> Self {
        Self(hash.0.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    #[test]
    fn test_serialize() {
        assert_tokens(
            &BoxHash::from(Box::<[u8]>::from(vec![0xde, 0xad, 0xbe, 0xef])).readable(),
            &[Token::BorrowedStr("deadbeef")],
        );

        assert_tokens(
            &BoxHash::from(Box::<[u8]>::from(vec![0xde, 0xad, 0xbe, 0xef])).compact(),
            &[
                Token::Seq { len: Some(4) },
                Token::U8(0xde),
                Token::U8(0xad),
                Token::U8(0xbe),
                Token::U8(0xef),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn hash_new() {
        let hash_value = [0xde, 0xad, 0xbe, 0xef];
        let hash = Hash::new(&hash_value);
        assert_eq!(*hash, hash_value);
    }

    #[test]
    fn hash_eq_slice() {
        let hash_value = [0xde, 0xad, 0xbe, 0xef];
        let hash = Hash::new(&hash_value);
        assert_eq!(*hash, hash_value[..]);
    }

    #[test]
    fn hash_from_box_hash() {
        let hash_value = [0xde, 0xad, 0xbe, 0xef];
        let box_hash = BoxHash::from(Box::<[u8]>::from(hash_value));
        let hash: &Hash = &box_hash;
        assert_eq!(*hash, hash_value);
    }

    #[test]
    fn box_hash_from_string() {
        let hash: BoxHash = "deadbeef".parse().unwrap();
        assert_eq!(hash, [0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn arc_hash_from_string() {
        let hash: ArcHash = "deadbeef".parse().unwrap();
        assert_eq!(hash, [0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn hash_to_string() {
        assert_eq!(Hash::from([0xde, 0xad, 0xbe, 0xef]).to_string(), "deadbeef");
    }

    #[test]
    fn box_hash_from() {
        let box_hash = BoxHash::from(&[12, 23, 56, 67]);
        assert_eq!(box_hash, [12, 23, 56, 67]);
    }

    #[test]
    fn box_hash_into() {
        let box_hash = BoxHash::from(&[12, 23, 56, 67]);
        let arc_hash: ArcHash = box_hash.clone().into();
        assert_eq!(box_hash, arc_hash);
        let rc_hash: RcHash = box_hash.clone().into();
        assert_eq!(box_hash, rc_hash);
    }

    #[test]
    fn hash_as_ref() {
        let hash_data = [1, 2, 3, 4];

        let box_hash = BoxHash::from(&hash_data);
        let hash: &Hash = box_hash.as_ref();
        assert_eq!(*hash, hash_data);

        let rc_hash = RcHash::from(box_hash.clone());
        let hash: &Hash = rc_hash.as_ref();
        assert_eq!(*hash, hash_data);

        let arc_hash = ArcHash::from(box_hash.clone());
        let hash: &Hash = arc_hash.as_ref();
        assert_eq!(*hash, hash_data);
    }
}
