use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    _bytes_through: usize,
    _reads: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {data: wrapped, _bytes_through: 0, _reads: 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self._bytes_through
    }

    pub fn reads(&self) -> usize {
        self._reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self._reads += 1;
        let result = self.data.read(buf);
        if let Ok(x) = result {
            self._bytes_through += x;
        }
        result
    }
}

pub struct WriteStats<W> {
    data: W,
    _bytes_through: usize,
    _writes: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {data: wrapped, _bytes_through: 0, _writes: 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self._bytes_through
    }

    pub fn writes(&self) -> usize {
        self._writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self._writes += 1;
        let result =  self.data.write(buf);
        if let Ok(x) = result {
            self._bytes_through += x;
        }
        result
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
