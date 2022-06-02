use std::net::Ipv4Addr;

use videohub::Hub;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running");
    let videohub = Hub::new(Ipv4Addr::new(10, 26, 135, 201), 9990);
    let hub_info = videohub.dump_hub_info()?;
    for line in hub_info.lines() {
        println!("{:?}", line);
    }

    Ok(())
}
