use std::io::Read;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub struct Chs {
    cylinders: u8,
    head: u8,
    sectors: u8
}

impl Chs {
    fn new(cylinders: u8, head: u8, sectors: u8) -> Self {
        Chs {
            cylinders, head, sectors
        }
    }

    pub fn to_lba(&self, heads: u8, sectors: u8) -> u32 {
        let c = self.cylinders as u32;
        let h = self.head as u32;
        let s = self.sectors as u32;
        (c * heads as u32 + h) * sectors as u32 + s - 1
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MbrAttributes {
    Bootable = 0x80,
    NonBootable = 0x00,
}

#[derive(Debug, Clone, Copy)]
pub struct MbrPartition {
    attributes: MbrAttributes,
    chs_start: Chs,
    p_type: u8,
    chs_end: Chs,
    lba_start: u32,
    total_sectors: u32
}

impl MbrPartition {
    pub fn new(data: [u8; 16]) -> Self {
        let start_bytes = [data[8], data[9], data[10], data[1]];
        let lba_start = u32::from_be_bytes(start_bytes);
        let end_bytes = [data[12], data[13], data[14], data[15]];
        let total_sectors = u32::from_be_bytes(end_bytes);
        MbrPartition {
            attributes: match data[0] {
                0x80 => MbrAttributes::Bootable,
                _ => MbrAttributes::NonBootable,
            },
            chs_start: Chs::new(data[1], data[2], data[3]),
            p_type: data[4],
            chs_end: Chs::new(data[5], data[6], data[7]),
            lba_start,
            total_sectors
        }
    }

    pub fn get_attributes_string(&self) -> String {
        match self.attributes {
            MbrAttributes::Bootable => "Bootable".to_string(),
            MbrAttributes::NonBootable => "Non-Bootable".to_string(),
        }
    }
}

pub struct MasterBootRecord {
    pub bootstrap: [u8; 440],
    pub disk_sig: u32,
    pub reserved: u16,
    pub partitions: [MbrPartition; 4],
    pub bootsector: u16,

    pub has_gpt: bool,
    data: [u8; 512]
}

impl MasterBootRecord {
    pub fn new(path: String) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer = [0; 512];
        file.read_exact(&mut buffer).unwrap();
        let mut partitions = [MbrPartition::new([0; 16]), MbrPartition::new([0; 16]), MbrPartition::new([0; 16]), MbrPartition::new([0; 16])];
        for i in 0..4 {
            let mut part = [0; 16];
            for j in 0..16 {
                part[j] = buffer[446 + i * 16 + j];
            }
            partitions[i] = MbrPartition::new(part);
        }
        MasterBootRecord {
            bootstrap: {
                let mut bs = [0; 440];
                for i in 0..440 {
                    bs[i] = buffer[i];
                }
                bs
            },
            disk_sig: u32::from_be_bytes([buffer[440], buffer[441], buffer[442], buffer[443]]),
            reserved: u16::from_be_bytes([buffer[444], buffer[445]]),
            partitions,
            bootsector: u16::from_be_bytes([buffer[510], buffer[511]]),
            has_gpt: partitions[0].p_type == 0xEE,
            data: buffer
        }
    }

    pub fn read_boot_record_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    pub fn get_partition_count(&self) -> usize {
        let mut count = 0;
        for i in 0..4 {
            if self.partitions[i].p_type != 0 {
                count += 1;
            }
        }
        count
    }

    pub fn get_partition(&self, index: usize) -> Option<MbrPartition> {
        if index < 4 {
            Some(self.partitions[index])
        } else {
            None
        }
    }

    pub fn get_lba(&self, index: usize) -> Option<u32> {
        if index < 4 {
            //Some(self.partitions[index].chs_end.to_lba();
            Some(self.partitions[index].lba_start) // This is wrong.
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_mbr() {
    }
}
