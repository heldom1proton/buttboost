use buttplug::client::ButtplugClient;
use buttplug::core::connector::ButtplugInProcessClientConnectorBuilder;
use buttplug::server::device::hardware::communication::btleplug::BtlePlugCommunicationManagerBuilder;
use buttplug::server::ButtplugServerBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = ButtplugServerBuilder::default()
        .name("Buttboost embedded buttplug server")
        .comm_manager(BtlePlugCommunicationManagerBuilder::default())
        .finish()?;
    let connector = ButtplugInProcessClientConnectorBuilder::default()
        .server(server)
        .finish();
    let client = ButtplugClient::new("Buttboost");
    client.connect(connector).await?;

    println!("Hello, world!");

    Ok(())
}
