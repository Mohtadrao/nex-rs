
/// Sum over an iterator of items castable to u32, wrapping on overflow.
pub fn sum_bytes_u32(data: impl IntoIterator<Item=u8>) -> u32 {
    data.into_iter().fold(0u32, |acc, b| acc.wrapping_add(b as u32))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sums() {
        assert_eq!(sum_bytes_u32([1,2,3,255]), 1+2+3+255);
    }
}
