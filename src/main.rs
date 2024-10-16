use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;
#[tokio::main]
async fn main() {
    get_serial_port_list();
    if let Ok(serial) = tokio_serial::new("com2", 115200).open_native_async() {
        let (mut serial_rx, mut serial_tx) = tokio::io::split(serial);
        let serial_rx_task = tokio::spawn(async move {
            let mut serial_rx_buf = bytes::BytesMut::new();
            serial_rx_buf.resize(1000, 0);
            loop {
                let rx_len = serial_rx.read(serial_rx_buf.as_mut()).await.unwrap();
                let _ = tokio::io::stdout()
                    .write(&serial_rx_buf[0..rx_len])
                    .await
                    .unwrap();
            }
        });
        let serial_tx_task = tokio::spawn(async move {
            let mut serial_tx_buf = bytes::BytesMut::new();
            serial_tx_buf.resize(1000, 0);
            loop {
                let rx_len = tokio::io::stdin()
                    .read(serial_tx_buf.as_mut())
                    .await
                    .unwrap();
                let _ = serial_tx.write(&serial_tx_buf[0..rx_len]).await.unwrap();
            }
        });
        let _ = tokio::join!(serial_rx_task, serial_tx_task);
    } else {
        println!("open serial error");
    }
}
fn get_serial_port_list() {
    let serial_list = tokio_serial::available_ports().unwrap();
    for serial_info in serial_list {
        println!("{}", serial_info.port_name);
    }
}
