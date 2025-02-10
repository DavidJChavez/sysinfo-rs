use sysinfo::{Disks, Networks, System};

fn main() {
    let sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let networks = Networks::new_with_refreshed_list();

    println!("OS:\t\t{}", System::long_os_version().unwrap_or_default());
    println!("Total memory:\t{} bytes", sys.total_memory());
    println!("Used memory:\t{} bytes", sys.used_memory());
    println!("Total swap:\t{} bytes", sys.total_swap());
    println!("Used swap:\t{} bytes", sys.used_swap());
    println!("Cores:\t\t{}", sys.physical_core_count().unwrap_or_default());
    println!("Logical procs:\t{}", sys.cpus().len());
    println!("Frequency:\t{}", sys.cpus()[0].frequency());
    for (index, disk) in disks.list().iter().enumerate() {
        println!("Disk {}", index + 1);
        println!("\tName:\t\t{}", disk.name().to_string_lossy());
        println!("\tTotal space:\t{}", disk.total_space());
        println!("\tFree space:\t{}", disk.available_space());
        println!("\tFile system:\t{}", disk.file_system().to_string_lossy());
        println!("\tKind:\t\t{}", disk.kind().to_string());
        println!("\tMount point:\t{}", disk.mount_point().to_string_lossy());
    }
    for (index, (interface_name, network)) in networks.list().iter().enumerate() {
        println!("Network {}", index + 1);
        println!("\tInterface name:\t{}", interface_name);
        println!("\tMAC address: \t{}", network.mac_address())
    }
}
