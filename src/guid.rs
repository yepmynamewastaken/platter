use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    pub fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Guid {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.data1, self.data2, self.data3, self.data4[0], self.data4[1], self.data4[2],
            self.data4[3], self.data4[4], self.data4[5], self.data4[6], self.data4[7]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid() {
        let guid = Guid::new(0x12345678, 0x9ABC, 0xDEF0, [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
        assert_eq!(guid.to_string(), "12345678-9ABC-DEF0-1234-567890ABCDEF");
    }
}
