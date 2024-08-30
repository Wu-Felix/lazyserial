pub mod serial {
    use builder::Builder;

    #[derive(Debug, Builder)]
    pub struct Serial {
        pub baud: Option<u32>,
        pub port: Option<String>,
    }
    pub trait SerialInfo {
        fn print(&self);
        fn serial_port_list(&self);
    }
    impl<U> SerialInfo for U
    where
        U: std::fmt::Debug,
    {
        fn print(&self) {
            println!("{:#?}", self);
        }
        fn serial_port_list(&self) {
            match serialport::available_ports() {
                Ok(serial_port_info) => {
                    let serial_name: Vec<_> = serial_port_info
                        .iter()
                        .map(|f| &f.port_name)
                        .zip(serial_port_info.iter().map(|f| {
                            if let serialport::SerialPortType::UsbPort(serialport::UsbPortInfo {
                                product,
                                ..
                            }) = &f.port_type
                            {
                                return product.to_owned();
                            }
                            None
                        }))
                        .collect();
                    eprintln!("{:#?}", serial_name);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    impl Serial {}
}
