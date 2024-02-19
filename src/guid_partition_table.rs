use std::io::{Read, Seek};
use crate::guid::Guid;

#[derive(Debug, Clone)]
pub struct GptPartition {
    pub partition_type_guid: Guid,
    pub unique_partition_guid: Guid,
    pub starting_lba: u64,
    pub ending_lba: u64,
    pub attributes: u64,
    pub partition_name: String
}

pub struct GuidPartitionTable {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub header_crc32: u32,
    pub reserved: u32,
    pub my_lba: u64,
    pub alternate_lba: u64,
    pub first_usable_lba: u64,
    pub last_usable_lba: u64,
    pub disk_guid: Guid,
    pub partition_entries_lba: u64,
    pub number_of_partition_entries: u32,
    pub size_of_partition_entry: u32,
    pub partition_entry_array_crc32: u32,
    pub partitions: Vec<GptPartition>
}

impl GuidPartitionTable {
    pub fn new(path: String, lba_size: u32) -> Self {
        println!("Reading GPT from {} with LBA size: {}", path, lba_size);
        // Open File and go to lba_size to get the GPT
        let mut file = std::fs::File::open(path).unwrap();
        file.seek(std::io::SeekFrom::Start(lba_size as u64)).unwrap();
        let mut buffer = [0; 512];
        file.read_exact(&mut buffer).unwrap();

        println!("Read data into buffer");

        let signature = u64::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]]);
        let revision = u32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
        let header_size = u32::from_le_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]);
        let header_crc32 = u32::from_le_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]);
        let reserved = u32::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]);
        let my_lba = u64::from_le_bytes([buffer[24], buffer[25], buffer[26], buffer[27], buffer[28], buffer[29], buffer[30], buffer[31]]);
        let alternate_lba = u64::from_le_bytes([buffer[32], buffer[33], buffer[34], buffer[35], buffer[36], buffer[37], buffer[38], buffer[39]]);
        let first_usable_lba = u64::from_le_bytes([buffer[40], buffer[41], buffer[42], buffer[43], buffer[44], buffer[45], buffer[46], buffer[47]]);
        let last_usable_lba = u64::from_le_bytes([buffer[48], buffer[49], buffer[50], buffer[51], buffer[52], buffer[53], buffer[54], buffer[55]]);
        let disk_guid = Guid::new(u32::from_le_bytes([buffer[56], buffer[57], buffer[58], buffer[59]]), u16::from_le_bytes([buffer[60], buffer[61]]), u16::from_le_bytes([buffer[62], buffer[63]]), [buffer[64], buffer[65], buffer[66], buffer[67], buffer[68], buffer[69], buffer[70], buffer[71]]);
        let partition_entries_lba = u64::from_le_bytes([buffer[72], buffer[73], buffer[74], buffer[75], buffer[76], buffer[77], buffer[78], buffer[79]]);
        let number_of_partition_entries = u32::from_le_bytes([buffer[80], buffer[81], buffer[82], buffer[83]]);
        let size_of_partition_entry = u32::from_le_bytes([buffer[84], buffer[85], buffer[86], buffer[87]]);
        let partition_entry_array_crc32 = u32::from_le_bytes([buffer[88], buffer[89], buffer[90], buffer[91]]);
        let mut partitions = Vec::new();

        for i in 0..number_of_partition_entries {
            let mut part = [0; 128];
            file.seek(std::io::SeekFrom::Start((partition_entries_lba * lba_size as u64) + (i as u64 * size_of_partition_entry as u64))).unwrap();
            file.read_exact(&mut part).unwrap();
            let partition_type_guid = Guid::new(u32::from_le_bytes([part[0], part[1], part[2], part[3]]), u16::from_le_bytes([part[4], part[5]]), u16::from_le_bytes([part[6], part[7]]), [part[8], part[9], part[10], part[11], part[12], part[13], part[14], part[15]]);
            let unique_partition_guid = Guid::new(u32::from_le_bytes([part[16], part[17], part[18], part[19]]), u16::from_le_bytes([part[20], part[21]]), u16::from_le_bytes([part[22], part[23]]), [part[24], part[25], part[26], part[27], part[28], part[29], part[30], part[31]]);
            let starting_lba = u64::from_le_bytes([part[32], part[33], part[34], part[35], part[36], part[37], part[38], part[39]]);
            let ending_lba = u64::from_le_bytes([part[40], part[41], part[42], part[43], part[44], part[45], part[46], part[47]]);
            let attributes = u64::from_le_bytes([part[48], part[49], part[50], part[51], part[52], part[53], part[54], part[55]]);
            // let partition_name = String::from_utf8_lossy(&part[56..120]).to_string();
            let partition_name = String::from_utf16(&[u16::from_le_bytes([part[56], part[57]]), u16::from_le_bytes([part[58], part[59]]), u16::from_le_bytes([part[60], part[61]]), u16::from_le_bytes([part[62], part[63]]), u16::from_le_bytes([part[64], part[65]]), u16::from_le_bytes([part[66], part[67]]), u16::from_le_bytes([part[68], part[69]]), u16::from_le_bytes([part[70], part[71]]), u16::from_le_bytes([part[72], part[73]])]).unwrap();
            partitions.push(GptPartition {
                partition_type_guid,
                unique_partition_guid,
                starting_lba,
                ending_lba,
                attributes,
                partition_name
            });
        }

        println!("Read GPT OK");

        GuidPartitionTable {
            signature,
            revision,
            header_size,
            header_crc32,
            reserved,
            my_lba,
            alternate_lba,
            first_usable_lba,
            last_usable_lba,
            disk_guid,
            partition_entries_lba,
            number_of_partition_entries,
            size_of_partition_entry,
            partition_entry_array_crc32,
            partitions
        }

    }

    pub fn get_partition_count(&self) -> usize {
        self.partitions.len()
    }

    pub fn get_partitions(&self) -> Vec<GptPartition> {
        self.partitions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpt() {
        // let gpt = GuidPartitionTable::new("tests/fixtures/gpt.img".to_string(), 512);
        // assert_eq!(gpt.get_partition_count(), 3);
        // let partitions = gpt.get_partitions();
        // assert_eq!(partitions[0].partition_name, "EFI System Partition");
        // assert_eq!(partitions[1].partition_name, "Microsoft Reserved Partition");
        // assert_eq!(partitions[2].partition_name, "Basic Data Partition");
    }
}
