use crate::guid_partition_table::{GptPartition, GuidPartitionTable};
use crate::master_boot_record::{MasterBootRecord, MbrPartition};

pub struct Disk {
    path: String,
}

impl Disk {
    pub fn new(path: String) -> Self {
        Disk { path }
    }

    pub fn open(&self) -> Result<OpenedDisk, std::io::Error> {
        let mbr = MasterBootRecord::new(self.path.clone());

        if mbr.has_gpt {
            // TODO: Use 512 for now, but we should use the actual LBA size
            let gpt = Some(GuidPartitionTable::new(self.path.clone(), 512));
            return Ok(OpenedDisk::new(mbr, gpt));
        }
        Ok(OpenedDisk::new(mbr, None))
    }
}

enum PartitionScheme {
    Mbr,
    Gpt,
}

pub enum Partition {
    Mbr(MbrPartition),
    Gpt(GptPartition),
}

pub struct OpenedDisk {
    scheme: PartitionScheme,
    mbr: MasterBootRecord,
    pub gpt: Option<GuidPartitionTable>,
}

impl OpenedDisk {
    pub fn new(mbr: MasterBootRecord, gpt: Option<GuidPartitionTable>) -> Self {
        OpenedDisk {
            scheme: match gpt {
                Some(_) => PartitionScheme::Gpt,
                None => PartitionScheme::Mbr,
            },
            mbr,
            gpt,
        }
    }

    pub fn read_boot_record_to_file(&self, path: &str) -> Result<(), std::io::Error> {

        match self.scheme {
            PartitionScheme::Mbr => self.mbr.read_boot_record_to_file(path),
            PartitionScheme::Gpt => self.gpt.as_ref().unwrap().read_boot_record_to_file(path),
        }
    }

    pub fn get_partition_count(&self) -> usize {
        match self.scheme {
            PartitionScheme::Mbr => self.mbr.get_partition_count(),
            PartitionScheme::Gpt => self.gpt.as_ref().unwrap().get_partition_count(),
        }
    }

    pub fn get_partition(&self, index: usize) -> Option<Partition> {
        match self.scheme {
            PartitionScheme::Mbr => {
                if index < self.get_partition_count() {
                    Some(Partition::Mbr(self.mbr.get_partition(index).unwrap()))
                } else {
                    None
                }
            }
            PartitionScheme::Gpt => {
                if let Some(gpt) = &self.gpt {
                    if index < gpt.get_partition_count() {
                        Some(Partition::Gpt(gpt.partitions[index].clone()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn get_attributes(&self, index: usize) -> Option<u64> {
        match self.scheme {
            PartitionScheme::Mbr => {
                if index < self.get_partition_count() {
                    None //self.mbr.partitions[index].get_attributes_string();
                } else {
                    None
                }
            }
            PartitionScheme::Gpt => {
                if let Some(gpt) = &self.gpt {
                    if index < gpt.get_partition_count() {
                        Some(gpt.partitions[index].attributes)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbr() {
        //    let disk = Disk::new("tests/fixtures/mbr.img".to_string());
        //     let opened_disk = disk.open().unwrap();
        //     assert_eq!(opened_disk.get_partition_count(), 2);
        //     assert_eq!(opened_disk.get_partition(0).unwrap().get_name(), "Primary");
        //     assert_eq!(opened_disk.get_partition(1).unwrap().get_name(), "Extended");
        // }
    }
    #[test]
    fn test_gpt() {
        //     let disk = Disk::new("tests/fixtures/gpt.img".to_string());
        //     let opened_disk = disk.open().unwrap();
        //     assert_eq!(opened_disk.get_partition_count(), 3);
        //     assert_eq!(opened_disk.get_partition(0).unwrap().get_name(), "EFI System");
        //     assert_eq!(opened_disk.get_partition(1).unwrap().get_name(), "Linux filesystem");
        //     assert_eq!(opened_disk.get_partition(2).unwrap().get_name(), "Linux swap");
    }
}
