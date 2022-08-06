use tonic::{Request};
pub mod netavarkproxy {
    tonic::include_proto!("netavark_proxy");
}

use netavarkproxy::netavark_proxy_client::NetavarkProxyClient;
use netavarkproxy::{NetworkConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NetavarkProxyClient::connect("http://[::1]:10000").await?;
    let response = client.get_lease(
        Request::new(NetworkConfig {
            iface: String::from("macvlan")
        }
    ))
        .await?;
    println!("Response {:?}", response);
    Ok(())
}
