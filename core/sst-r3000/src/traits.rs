use std::io;

pub trait ReadBytesExt: io::Read {
    fn read_le_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0; std::mem::size_of::<i32>()];
        self.read_exact(&mut buf)?;

        Ok(i32::from_le_bytes(buf))
    }

    fn read_le_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0; std::mem::size_of::<u32>()];
        self.read_exact(&mut buf)?;

        Ok(u32::from_le_bytes(buf))
    }

    fn read_le_i64(&mut self) -> io::Result<i64> {
        let mut buf = [0; std::mem::size_of::<i64>()];
        self.read_exact(&mut buf)?;

        Ok(i64::from_le_bytes(buf))
    }
}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}
