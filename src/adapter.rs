use core::fmt::Arguments;
pub(crate) struct FmtIoWriter<W: std::io::Write>(W);

impl<W> FmtIoWriter<W>
where
    W: std::io::Write,
{
    pub(crate) fn new(buf: W) -> Self {
        Self(buf)
    }
}

impl<W> std::fmt::Write for FmtIoWriter<W>
where
    W: std::io::Write,
{
    fn write_str(&mut self, input: &str) -> Result<(), std::fmt::Error> {
        match self.0.write(input.as_bytes()) {
            Err(_) => Err(std::fmt::Error),
            _ => match self.0.flush() {
                Err(_) => Err(std::fmt::Error),
                _ => Ok(()),
            },
        }
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), std::fmt::Error> {
        match self.0.write_fmt(args) {
            Err(_) => Err(std::fmt::Error),
            _ => Ok(()),
        }
    }
}
