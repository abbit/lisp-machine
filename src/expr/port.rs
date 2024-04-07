use core::fmt;
#[cfg(unix)]
use std::os::fd::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Stdin, Stdout, Write},
    path::Path,
};

pub trait InputPortTrait {
    fn peek(&mut self) -> io::Result<char>;
    fn read_char(&mut self) -> io::Result<char>;
    fn read_string(&mut self) -> io::Result<String>;
    fn close(&mut self) -> io::Result<()>;
    fn is_closed(&self) -> bool;
}

pub trait InputPortSuperTrait: InputPortTrait + std::fmt::Debug + std::fmt::Display {}

pub trait OutputPortTrait: Write {
    fn close(&mut self) -> io::Result<()>;
    fn is_closed(&self) -> bool;
}

pub trait OutputPortSuperTrait: OutputPortTrait + std::fmt::Debug + std::fmt::Display {}

fn read_u8(reader: &mut impl Read) -> io::Result<u8> {
    let mut byte_buf = [0_u8];
    let num_read = reader.read(&mut byte_buf)?;
    if num_read == 0 {
        Err(io::ErrorKind::UnexpectedEof.into())
    } else {
        Ok(byte_buf[0])
    }
}

fn read_char(reader: &mut impl Read) -> io::Result<char> {
    let mut buf = [0_u8; 4];
    for i in 0..4 {
        match read_u8(reader) {
            Ok(b) => buf[i] = b,
            Err(e) => {
                return if i != 0 && e.kind() == io::ErrorKind::UnexpectedEof {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "stream does not contain valid UTF-8",
                    ))
                } else {
                    Err(e)
                }
            }
        }

        if let Some(c) = char::from_u32(u32::from_le_bytes(buf)) {
            return Ok(c);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "stream does not contain valid UTF-8",
    ))
}

#[derive(Debug)]
struct PeekableBufReader<R: Read> {
    reader: BufReader<R>,
    peek_buffer: Option<char>,
}

impl<R: Read> PeekableBufReader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            peek_buffer: None,
        }
    }

    fn get_inderlying(&self) -> &R {
        self.reader.get_ref()
    }

    fn peek(&mut self) -> io::Result<char> {
        if let Some(c) = self.peek_buffer {
            Ok(c)
        } else {
            let c = read_char(&mut self.reader)?;
            self.peek_buffer = Some(c);
            Ok(c)
        }
    }

    fn read_char(&mut self) -> io::Result<char> {
        if let Some(c) = self.peek_buffer.take() {
            Ok(c)
        } else {
            read_char(&mut self.reader)
        }
    }

    fn read_string(&mut self) -> io::Result<String> {
        let mut str_buf = String::new();
        if let Some(c) = self.peek_buffer.take() {
            str_buf.push(c);
        }
        self.reader.read_to_string(&mut str_buf)?;
        let str_buf = str_buf.strip_suffix('\n').unwrap_or(&str_buf).to_string();
        Ok(str_buf)
    }
}

#[derive(Debug)]
struct ReaderInputPort<R: Read> {
    reader: Option<PeekableBufReader<R>>,
}

impl<R: Read> ReaderInputPort<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: Some(PeekableBufReader::new(reader)),
        }
    }

    fn reader_or_closed_err(&mut self) -> io::Result<&mut PeekableBufReader<R>> {
        if let Some(r) = self.reader.as_mut() {
            Ok(r)
        } else {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "port is closed"))
        }
    }

    fn get_inderlying(&self) -> Option<&R> {
        self.reader.as_ref().map(|r| r.get_inderlying())
    }
}

impl<R: Read> InputPortTrait for ReaderInputPort<R> {
    fn peek(&mut self) -> io::Result<char> {
        self.reader_or_closed_err()?.peek()
    }

    fn read_char(&mut self) -> io::Result<char> {
        self.reader_or_closed_err()?.read_char()
    }

    fn read_string(&mut self) -> io::Result<String> {
        self.reader_or_closed_err()?.read_string()
    }

    fn close(&mut self) -> io::Result<()> {
        self.reader = None;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.reader.is_none()
    }
}

#[derive(Debug)]
struct WriterOutputPort<W: Write> {
    writer: Option<BufWriter<W>>,
}

impl<W: Write> WriterOutputPort<W> {
    fn new(writer: W) -> Self {
        Self {
            writer: Some(BufWriter::new(writer)),
        }
    }

    fn writer_or_closed_err(&mut self) -> io::Result<&mut BufWriter<W>> {
        if let Some(r) = self.writer.as_mut() {
            Ok(r)
        } else {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "port is closed"))
        }
    }

    fn get_inderlying(&self) -> Option<&W> {
        self.writer.as_ref().map(|r| r.get_ref())
    }
}

impl<W: Write> Write for WriterOutputPort<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer_or_closed_err()?.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer_or_closed_err()?.flush()
    }
}

impl<W: Write> OutputPortTrait for WriterOutputPort<W> {
    fn close(&mut self) -> io::Result<()> {
        self.writer = None;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.writer.is_none()
    }
}

// StdinInputPort

#[derive(Debug)]
pub struct StdinInputPort {
    reader: PeekableBufReader<Stdin>,
}

impl StdinInputPort {
    pub fn new() -> Self {
        Self {
            reader: PeekableBufReader::new(io::stdin()),
        }
    }
}

impl PartialEq for StdinInputPort {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl InputPortTrait for StdinInputPort {
    fn peek(&mut self) -> io::Result<char> {
        self.reader.peek()
    }
    fn read_char(&mut self) -> io::Result<char> {
        self.reader.read_char()
    }
    fn read_string(&mut self) -> io::Result<String> {
        self.reader.read_string()
    }
    fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn is_closed(&self) -> bool {
        false
    }
}

impl fmt::Display for StdinInputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#<stdin input port>")
    }
}

impl InputPortSuperTrait for StdinInputPort {}

// FileInputPort

#[derive(Debug)]
pub struct FileInputPort {
    reader: ReaderInputPort<File>,
}

impl FileInputPort {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: ReaderInputPort::new(file),
        })
    }
}

impl PartialEq for FileInputPort {
    fn eq(&self, other: &Self) -> bool {
        match (self.reader.get_inderlying(), other.reader.get_inderlying()) {
            (Some(file1), Some(file2)) => {
                #[cfg(unix)]
                {
                    let fd1 = file1.as_raw_fd();
                    let fd2 = file2.as_raw_fd();
                    fd1 == fd2
                }
                #[cfg(windows)]
                {
                    let fd1 = file1.as_raw_handle();
                    let fd2 = file2.as_raw_handle();
                    fd1 == fd2
                }
            }
            _ => false,
        }
    }
}

impl InputPortTrait for FileInputPort {
    fn peek(&mut self) -> io::Result<char> {
        self.reader.peek()
    }
    fn read_char(&mut self) -> io::Result<char> {
        self.reader.read_char()
    }
    fn read_string(&mut self) -> io::Result<String> {
        self.reader.read_string()
    }
    fn close(&mut self) -> io::Result<()> {
        self.reader.close()
    }
    fn is_closed(&self) -> bool {
        self.reader.is_closed()
    }
}

impl fmt::Display for FileInputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reader.get_inderlying() {
            Some(file) => {
                #[cfg(unix)]
                let fd = file.as_raw_fd();
                #[cfg(windows)]
                let fd = file.as_raw_handle();
                write!(f, "#<file input port #{:?}>", fd)
            }
            None => write!(f, "#<closed file input port>"),
        }
    }
}

impl InputPortSuperTrait for FileInputPort {}

// StdoutOutputPort

#[derive(Debug)]
pub struct StdoutOutputPort {
    writer: WriterOutputPort<Stdout>,
}

impl StdoutOutputPort {
    pub fn new() -> Self {
        Self {
            writer: WriterOutputPort::new(io::stdout()),
        }
    }
}

impl PartialEq for StdoutOutputPort {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Write for StdoutOutputPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl OutputPortTrait for StdoutOutputPort {
    fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn is_closed(&self) -> bool {
        false
    }
}

impl fmt::Display for StdoutOutputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#<stdout output port>")
    }
}

impl OutputPortSuperTrait for StdoutOutputPort {}

// FileOutputPort

#[derive(Debug)]
pub struct FileOutputPort {
    writer: WriterOutputPort<File>,
}

impl FileOutputPort {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            writer: WriterOutputPort::new(file),
        })
    }
}

impl PartialEq for FileOutputPort {
    fn eq(&self, other: &Self) -> bool {
        match (self.writer.get_inderlying(), other.writer.get_inderlying()) {
            (Some(file1), Some(file2)) => {
                #[cfg(unix)]
                {
                    let fd1 = file1.as_raw_fd();
                    let fd2 = file2.as_raw_fd();
                    fd1 == fd2
                }
                #[cfg(windows)]
                {
                    let fd1 = file1.as_raw_handle();
                    let fd2 = file2.as_raw_handle();
                    fd1 == fd2
                }
            }
            _ => false,
        }
    }
}

impl Write for FileOutputPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl OutputPortTrait for FileOutputPort {
    fn close(&mut self) -> io::Result<()> {
        self.writer.close()
    }
    fn is_closed(&self) -> bool {
        self.writer.is_closed()
    }
}

impl fmt::Display for FileOutputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.writer.get_inderlying() {
            Some(file) => {
                #[cfg(unix)]
                let fd = file.as_raw_fd();
                #[cfg(windows)]
                let fd = file.as_raw_handle();
                write!(f, "#<file output port #{:?}>", fd)
            }
            None => write!(f, "#<closed file output port>"),
        }
    }
}

impl OutputPortSuperTrait for FileOutputPort {}
