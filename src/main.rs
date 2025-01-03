
use std::net::IpAddr;

use tracert;

fn main() {
    println!("Smart Network Troubleshooter");
    interface();
    routing();
}

fn interface() {
    match default_net::get_interfaces().is_empty() {
        true => eprintln!("interface::any::exists: false"),
        false => println!("interface::any::exists: true"),
    }

    match default_net::get_default_interface() {
        Ok(def_if) => {
            println!("interface::default::exists: true");
            println!("interface::default::name: {}", def_if.name);
            if def_if.is_broadcast() {
                println!("interface::default::broadcast: true");
            } else {
                eprintln!("interface::default::broadcast: false");
            }
            if !def_if.is_loopback() {
                println!("interface::default::loopback: false (desired)");
            } else {
                eprintln!("interface::default::loopback: true (undesired)");
            }
            if def_if.is_up() {
                println!("interface::default::up: true");
            } else {
                eprintln!("interface::default::up: false");
            }
        }
        Err(what) => {
            eprintln!("interface::default::exists: false");
            eprintln!("verbose error: {}", what);
        }
    }
}

fn routing() {

    let localhost: IpAddr = "127.0.0.1".parse().unwrap();
    let cloudflare_v4: IpAddr = "1.1.1.1".parse().unwrap();
    let cloudflare_v6: IpAddr = "2606:4700:4700::1111".parse().unwrap();
    let google_v4: IpAddr = "8.8.8.8".parse().unwrap();
    let google_v6: IpAddr = "2001:4860:4860::8888".parse().unwrap();
    let gateway = default_net::get_default_gateway();

    fn ping_and_log(name: &str, addr: IpAddr) {
        match tracert::ping::Pinger::new(addr) {
            Ok(pinger) => {
                println!("routing::ping::{}::pinger: true", name);
                match pinger.ping() {
                    Ok(result) => {
                        match result.status {
                            tracert::ping::PingStatus::Error => { eprintln!("routing::ping::{}::status: error", name) },
                            tracert::ping::PingStatus::Timeout => { eprintln!("routing::ping::{}::status: timeout", name) },
                            tracert::ping::PingStatus::Done => { println!("routing::ping::{}::status: done", name) },
                        }
                    },
                    Err(what) => {
                        eprintln!("routing::ping::{}::response: false", name);
                        eprintln!("verbose error: {}", what);
                    },
                }
            },
            Err(what) => {
                eprintln!("routing::ping::{}::pinger: false", name);
                eprintln!("verbose error: {}", what);
            },
        }
    }

    ping_and_log("localhost", localhost);

    if let Ok(gateway_unwr) = gateway {
        println!("routing::gateway::exists: true");
        ping_and_log("gateway", gateway_unwr.ip_addr);
    } else {
        eprintln!("routing::gateway::exists: false")
    }

    ping_and_log("cloudflare_v4", cloudflare_v4);
    ping_and_log("cloudflare_v6", cloudflare_v6);
    ping_and_log("google_v4", google_v4);
    ping_and_log("google_v6", google_v6);

}
