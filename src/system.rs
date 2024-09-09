use sysinfo::{Components, Disks, Networks, System};

pub fn hello_cpu() -> String {
    "Hi! I am you task tracker.".to_string()
}

/// Give a system summary
///
/// # Returns
///
/// None
pub fn sys_summary() {
    let mut system = System::new_all();

    // Refresh the system information
    system.refresh_all();

    println!("Total memory: {}", system.total_memory());
    println!("Total swap: {}", system.total_swap());
    println!("Total used: {}", system.used_memory());

    println!("System name: {:?}", System::name());
    println!("Kernel version: {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", system.cpus().len());

    // Display processes ID, name na disk usage:
    //for (pid, process) in system.processes() {
    //println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    //}

    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.
    }

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
}

pub fn cpu_usage() {
    let mut sys = System::new();
    loop {
        sys.refresh_cpu_usage(); // Refreshing CPU usage.
        for cpu in sys.cpus() {
            print!("{}% ", cpu.cpu_usage());
        }
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
