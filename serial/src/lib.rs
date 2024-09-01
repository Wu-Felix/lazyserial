pub mod serial {
    use builder::Builder;
    use serialport::SerialPort;
    use std::{
        io::{stdout, Write},
        time::Duration,
    };

    #[derive(Debug, Builder)]
    pub struct Serial {
        pub baud: Option<u32>,
        pub port: Option<String>,
    }
    pub trait SerialInfo {
        fn print(&self);
        fn serial_port_list();
    }
    impl<U> SerialInfo for U
    where
        U: std::fmt::Debug,
    {
        fn print(&self) {
            println!("{:#?}", self);
        }
        fn serial_port_list() {
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
    impl Serial {
        pub fn open(&self) {
            let serial = serialport::new(self.port.clone().unwrap(), self.baud.unwrap())
                .timeout(Duration::from_millis(100))
                .open();
            match serial {
                Ok(mut p) => {
                    let mut read_buf = vec![0; 1000];
                    loop {
                        let len = p.read(read_buf.as_mut_slice());
                        if let Ok(read_len) = len {
                            if let Err(e) = stdout().write_all(&read_buf[..read_len]) {
                                println!("{}", e);
                            }
                        }
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
