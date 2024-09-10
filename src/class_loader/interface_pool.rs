#[derive(Debug, Clone)]
pub struct InterfacePool {
    size: u16,
    interfaces: Vec<u16>,
}
impl InterfacePool {
    pub fn new(data: &[u8]) -> (InterfacePool, usize) {
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut interfaces = Vec::with_capacity(size as usize);
        if size > 0 {
            for i in 0..(size as usize) {
                interfaces.push(u16::from_be_bytes(
                    data[(i * 2)..((i * 2) + 2)].try_into().unwrap(),
                ));
            }
        }
        (InterfacePool { size, interfaces }, (size as usize * 2) + 2)
    }
}
