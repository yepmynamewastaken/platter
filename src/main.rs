
mod disk;
mod master_boot_record;
mod guid;
mod guid_partition_table;

fn main() {

    let args = std::env::args().collect::<Vec<String>>();

    let disk = disk::Disk::new(String::from(&args[1]));
    let opened_disk = disk.open();

    match opened_disk {
        Ok(disk) => {
            match disk.get_partition(0) {

                Some(partition) => {
                    match partition {
                        disk::Partition::Mbr(part) => {
                            println!("Mbr Partition: {:?}", part);
                        },
                        disk::Partition::Gpt(part) => {
                            println!("Gpt Partition: {:?}", part);
                            println!("Partition Type GUID: {}", part.partition_type_guid);
                        }
                    }
                },
                None => {
                    println!("No partition found");
                }
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

}
