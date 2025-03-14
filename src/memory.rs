pub struct Memory{
    data: Vec<u8>, //Byte array
}
impl Memory{
    pub fn new(size: usize) -> Self{
        Memory{
            data: vec![0;size], //initialising memory with size number of bytes
        }
    }
    
    pub fn read_byte(&self, address: u32) -> u8{
        if address < self.data.len() as u32{
            self.data[address as usize]
        }else{
            panic!("memory usage read out of bound at address 0x{:X} ", address);
        }
    }

    // Function to read 4 bytes (a word) from memory at a given address (Little-Endian)

    pub fn read_word(&self, address: u32) -> u32{
        if address + 3 < self.data.len() as u32{ //Check for bounds
            let byte0 = self.read_byte(address) as u32;
            let byte1 = self.read_byte(address +1) as u32;
            let byte2 = self.read_byte(address +2) as u32;
            let byte3 = self.read_byte(address +3) as u32;
            byte0 | (byte1 <<8) | (byte2 <<16) | (byte3 <<24)
        }else{
            panic!("Memory word out of bounds at address 0x{:X}", address);
        }
    }
    pub fn write_bytes(&mut self, address: u32, bytes: &[u8]) {
        let end = address as usize + bytes.len();
        if end > self.data.len() {
            panic!("Memory write out of bounds at address 0x{:X}", address);
        }
        self.data[address as usize .. end].copy_from_slice(bytes);
    }

}
