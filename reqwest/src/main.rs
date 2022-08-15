use videohub::protocol::DeviceInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let s: DeviceInfo = client
        .get("http://192.168.1.102:8000/device_info")
        .header(reqwest::header::CONTENT_TYPE, "json")
        .send()
        .await?
        .json()
        .await?;
    // let resp: DeviceInfo = reqwest::get("http://127.0.0.1:8000/device_info")
    //     .await?
    //     .json()
    //     .await?;
    println!("{:?}", s);
    Ok(())
}
