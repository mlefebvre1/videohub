use std::net::Ipv4Addr;

use videohub::Hub;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running");
    let videohub = Hub::new(Ipv4Addr::new(10, 26, 135, 201), 9990);
    let content = videohub.dump_hub_info()?;
    let hub_info = videohub.get_hub_info(&content)?;
    println!("{:?}", hub_info);
    // for line in content.lines() {
    //     println!("{:?}", line);
    // }

    Ok(())
}
