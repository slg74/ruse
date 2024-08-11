use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt, CpuExt};
use sysinfo::NetworkExt;


fn get_cpu_usage() {
    println!("\nUtilization:");
    let mut sys = System::new_all();
    sys.refresh_cpu();
    thread::sleep(Duration::from_millis(500)); 
    sys.refresh_cpu(); 
    let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    println!("CPU Usage: {:.1}%", cpu_usage);
}

fn get_cpu_saturation() {
    println!("\nSaturation:"); 
    let mut sys: System = System::new_all();
    sys.refresh_all();
    let load_avg = sys.load_average();
    println!("CPU Load Average (1 minute): {:.2}", load_avg.one); 
    println!("CPU Load Average (5 minutes): {:.2}", load_avg.five); 
    println!("CPU Load Average (15 minutes): {:.2}", load_avg.fifteen); 
}

fn get_network_errors() {
    println!("\nErrors:");
    let mut sys: System = System::new_all();
    sys.refresh_networks();

    let mut total_errors = 0;
    for (interface_name, data) in sys.networks() {
        let errors = data.total_errors_on_received();
        total_errors += errors;
        println!("Network Errors on {}: {}", interface_name, errors);
    }
    println!("Network Errors: {}", total_errors); 
}

fn main() {
    println!("USE method utility - cpu usage, saturation, network errors");
    println!("----------------------------------------------------------"); 

    get_cpu_usage();
    get_cpu_saturation();
    get_network_errors(); 
}