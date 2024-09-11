use super::Exception;
#[derive(Debug, Clone)]
pub struct ExceptionTablePool {
    length: u16,
    exceptions_table: Box<[Exception]>,
}
impl ExceptionTablePool {
    pub fn new(data: &[u8]) -> (ExceptionTablePool, u32) {
        let length = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut exceptions_table = Vec::with_capacity(length as usize);
        for i in 0..(length as usize) {
            exceptions_table.push(Exception::new(&data[2 + (i * 8)..10 + (i * 8)]));
        }
        let exceptions_table = exceptions_table.into_boxed_slice();
        (
            ExceptionTablePool {
                length,
                exceptions_table,
            },
            (length as u32 * 8) + 2,
        )
    }
}
