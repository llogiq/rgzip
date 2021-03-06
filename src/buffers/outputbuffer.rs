use errors::GzipResult;

pub trait OutputBuffer {
    fn put_u8(&mut self, data: u8) -> GzipResult<()>;

    fn put_data(&mut self, data: Vec<u8>) -> GzipResult<()> {
        for d in data {
            self.put_u8(d)?
        }
        Ok(())
    }

    fn copy_window(&mut self, distance: u32, length: u32) -> GzipResult<()>;
}


