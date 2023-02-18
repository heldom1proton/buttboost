use std::str::FromStr;
use buttplug::client::{ButtplugClient, ButtplugClientEvent};
use buttplug::core::connector::ButtplugInProcessClientConnectorBuilder;
use buttplug::server::device::hardware::communication::btleplug::BtlePlugCommunicationManagerBuilder;
use buttplug::server::ButtplugServerBuilder;
use futures::StreamExt;
use tokio::io::{AsyncBufReadExt, BufReader, stdin};


async fn wait_for_input() -> anyhow::Result<Option<String>> {
    Ok(BufReader::new(stdin())
        .lines()
        .next_line()
        .await?)
}

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
    let mut events = client.event_stream();

    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            match event {
                ButtplugClientEvent::DeviceAdded(device) => {
                    println!("Device {} Connected!", device.name());
                }
                ButtplugClientEvent::DeviceRemoved(info) => {
                    println!("Device {} Removed!", info.name());
                }
                ButtplugClientEvent::ScanningFinished => {
                    println!("Device scanning is finished!");
                }
                other => println!("Other event received: {other:?}"),
            }
        }
    });

    println!("Scanning for devices. Press [ENTER] to finish.");
    client.start_scanning().await?;
    wait_for_input().await?;

    println!("Stopping scanning...");
    client.stop_scanning().await?;

    let devices = client.devices();
    if devices.is_empty() {
        println!("Found no devices, exiting");
        return Ok(());
    }

    println!("Choose a device by entering its number:");
    for (index, device) in devices.iter().enumerate() {
        println!("{index} - {}", device.name());
    }
    let _device = loop {
        let Some(Ok(choice)) = wait_for_input().await?.map(|choice| usize::from_str(&choice)) else {
            println!("Invalid choice, enter a non-negative integer");
            continue;
        };
        let Some(device) = devices.get(choice) else {
            println!("Choice too large, enter a device number from the list above");
            continue;
        };
        break device;
    };

    Ok(())
}
