use std::time::Duration;

pub struct SerialInterface {
    port_instance: Box<dyn serialport::SerialPort + 'static>,
    port_name: String,
}

impl SerialInterface {
    pub fn new(port_name: &str, baudrate: u32, timeout: Duration) -> SerialInterface {
        let port = serialport::new(port_name, baudrate)
            .timeout(timeout)
            .open()
            .expect(format!("Failed to open port: {}", port_name).as_str());

        SerialInterface {
            port_instance: port,
            port_name: port_name.to_string(),
        }
    }

    /// Отправка данных на интерфейсную плату
    pub fn write_data(&mut self, data: &[u8]) -> Result<usize, String> {
        if let Ok(size) = self.port_instance.write(data) {
            return Ok(size);
        }
        return Err(format!("Failed to write to port: {}", self.port_name));
    }

    /// Очистка входного буфера приемника
    pub fn clear_input_buffer(&mut self) -> Result<(), String> {
        if let Ok(_) = self.port_instance.clear(serialport::ClearBuffer::All) {
            return Ok(());
        }
        return Err(format!("Failed to clear input buffer: {}", self.port_name));
    }


    /// Чтение данных от интерфейсной платы
    pub fn read_data(&mut self, data: &mut [u8]) -> Result<usize, String> {

        //read_to_end(&mut self, dest_vec: &mut Vec<u8>) -> io::Result<usize>
        if let Ok(size) = self.port_instance.read_exact(data)  {
            return Ok(1_usize);
        }
        return Err("IMv2: Timeout has been reached".to_string());
    }
}
