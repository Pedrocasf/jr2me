#[derive(Debug, Clone, Copy)]
pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}
impl Exception {
    pub fn new(data: &[u8]) -> Exception {
        Exception {
            start_pc: u16::from_be_bytes(data[0..2].try_into().unwrap()),
            end_pc: u16::from_be_bytes(data[2..4].try_into().unwrap()),
            handler_pc: u16::from_be_bytes(data[4..6].try_into().unwrap()),
            catch_type: u16::from_be_bytes(data[6..8].try_into().unwrap()),
        }
    }
}
