//! Signature method (notes from Wii U Xenoblade reversing in nex-go docs).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
pub enum SignatureMethod {
    Method0 = 0,
    UseAddress = 1,
    Method2 = 2,
    Method3 = 3,
    Method4 = 4,
    Method5 = 5,
    UseKey = 6,
    Method7 = 7,
    UseEntropy = 8,
    Ignore = 9,
}
