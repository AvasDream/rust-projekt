extern crate pnet;
extern crate clap;
extern crate rand;

use std::net::Ipv4Addr;

mod utility;
mod tcp_scans;
mod icmp_scan;
mod iana_mapping;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum ScanType{
     TcpFull,
     Ping,
 }
pub struct ScanResult {
    ports: Vec<u16>,
    ip: Ipv4Addr,
    scantype: ScanType,
    is_up: bool
}
impl Clone for ScanResult {
    fn clone(&self) -> ScanResult {
        let clone = ScanResult {
            ports: self.ports.clone(),
            scantype: self.scantype.clone(),
            ip: self.ip.clone(),
            is_up: self.is_up.clone(),
        };
        clone
    }
}
pub struct ScanConfig {
    ips: Vec<Ipv4Addr>,
    start_port: u16,
    end_port: u16,
    scantype: ScanType,
    to_file: String,
}
impl Clone for ScanConfig {
    fn clone(&self) -> ScanConfig {
        let clone = ScanConfig {
            ips: self.ips.clone(),
            start_port: self.start_port.clone(),
            end_port: self.end_port.clone(),
            scantype: self.scantype,
            to_file: self.to_file.clone(),
        };
        clone
    }
}
fn main() {

    let scanconfig = utility::get_config();


    let mut output = "".to_string();
    println!("Start scan...");
    match scanconfig.scantype {
        ScanType::TcpFull => {
            let results = tcp_scans::tcp_scan(scanconfig.clone());
            println!("Preparing output...");
            output = utility::prepare_output(results);
        },
        ScanType::Ping => {
            let results = icmp_scan::ping_scan(scanconfig.clone());
            println!("Preparing output...");
            output = utility::prepare_output(results);
        },
    }
    if scanconfig.to_file != "" {
        println!("{}",output);
        utility::write_to_file(scanconfig.to_file, output);
    } else {
        println!("{}",output);
    }
    println!("Scan finished.");
}







