use clap::Parser;
use serial::serial::SerialInfo;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    serial: Option<SerialFn>,
}
#[derive(clap::Subcommand)]
enum SerialFn {
    List {
        list: Option<u8>,
    },
    Serial {
        ///端口设置
        port: Option<String>,
        ///波特率设置
        #[arg(long, short)]
        baud: Option<u32>,
    },
}

fn main() {
    if let Some(cli) = Cli::parse().serial {
        match cli {
            SerialFn::List { list } => {
                let _ = list;
                serial::serial::Serial::serial_port_list();
            }
            SerialFn::Serial { port, baud } => {
                if let Some(port) = port {
                    let serial = serial::serial::Serial::builder()
                        .port(port)
                        .baud(baud.unwrap())
                        .build()
                        .unwrap();
                    serial.open();
                }
            }
        }
    }
}
