use sysinfo::{Disks, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();

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
        println!("\tName:\t\t{:?}", disk.name());
        println!("\tTotal space:\t{}", disk.total_space());
    }
}
