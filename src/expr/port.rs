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

pub trait OutputPortTrait: Write {
    fn close(&mut self) -> io::Result<()>;
    fn is_closed(&self) -> bool;
}

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
pub struct PeekableBufReader<R: Read> {
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
pub struct ReaderInputPort<R: Read> {
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

#[derive(Debug)]
pub enum InputPort {
    Stdin(PeekableBufReader<Stdin>),
    File(ReaderInputPort<File>),
}

impl InputPort {
    pub fn new_stdin() -> Self {
        Self::Stdin(PeekableBufReader::new(io::stdin()))
    }

    pub fn new_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self::File(ReaderInputPort::new(File::open(path)?)))
    }
}

impl PartialEq for InputPort {
    fn eq(&self, _other: &Self) -> bool {
        panic!("trying to compare input port");
    }
}

impl InputPortTrait for InputPort {
    fn peek(&mut self) -> io::Result<char> {
        match self {
            Self::Stdin(port) => port.peek(),
            Self::File(port) => port.peek(),
        }
    }
    fn read_char(&mut self) -> io::Result<char> {
        match self {
            Self::Stdin(port) => port.read_char(),
            Self::File(port) => port.read_char(),
        }
    }
    fn read_string(&mut self) -> io::Result<String> {
        match self {
            Self::Stdin(port) => port.read_string(),
            Self::File(port) => port.read_string(),
        }
    }

    fn close(&mut self) -> io::Result<()> {
        match self {
            Self::Stdin(_) => Ok(()),
            Self::File(port) => port.close(),
        }
    }
    fn is_closed(&self) -> bool {
        match self {
            Self::Stdin(_) => false,
            Self::File(port) => port.is_closed(),
        }
    }
}

impl fmt::Display for InputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdin(_) => write!(f, "#<stdin input port>"),
            Self::File(port) => match port.get_inderlying() {
                Some(file) => {
                    #[cfg(unix)]
                    let fd = file.as_raw_fd();
                    #[cfg(windows)]
                    let fd = file.as_raw_handle();
                    write!(f, "#<file input port #{:?}>", fd)
                }
                None => write!(f, "#<closed file input port>"),
            },
        }
    }
}

#[derive(Debug)]
pub enum OutputPort {
    Stdout(WriterOutputPort<Stdout>),
    File(WriterOutputPort<File>),
}

impl OutputPort {
    pub fn new_stdout() -> Self {
        Self::Stdout(WriterOutputPort::new(io::stdout()))
    }

    pub fn new_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self::File(WriterOutputPort::new(File::create(path)?)))
    }
}

impl PartialEq for OutputPort {
    fn eq(&self, _other: &Self) -> bool {
        panic!("trying to compare output port");
    }
}

impl Write for OutputPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Stdout(port) => port.write(buf),
            Self::File(port) => port.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(port) => port.flush(),
            Self::File(port) => port.flush(),
        }
    }
}

impl OutputPortTrait for OutputPort {
    fn close(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(_) => Ok(()),
            Self::File(port) => port.close(),
        }
    }

    fn is_closed(&self) -> bool {
        match self {
            Self::Stdout(_) => false,
            Self::File(port) => port.is_closed(),
        }
    }
}

impl fmt::Display for OutputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdout(_) => write!(f, "#<stdout output port>"),
            Self::File(port) => match port.get_inderlying() {
                Some(file) => {
                    #[cfg(unix)]
                    let fd = file.as_raw_fd();
                    #[cfg(windows)]
                    let fd = file.as_raw_handle();
                    write!(f, "#<file output port #{:?}>", fd)
                }
                None => write!(f, "#<closed file output port>"),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Port {
    Input(InputPort),
    Output(OutputPort),
}

impl Port {
    pub fn new_stdin() -> Self {
        Self::Input(InputPort::new_stdin())
    }

    pub fn new_stdout() -> Self {
        Self::Output(OutputPort::new_stdout())
    }

    pub fn new_input_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self::Input(InputPort::new_file(path)?))
    }

    pub fn new_output_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self::Output(OutputPort::new_file(path)?))
    }

    pub fn is_input(&self) -> bool {
        matches!(self, Self::Input(_))
    }

    pub fn is_output(&self) -> bool {
        matches!(self, Self::Output(_))
    }

    pub fn as_input(&mut self) -> Option<&mut InputPort> {
        match self {
            Self::Input(port) => Some(port),
            _ => None,
        }
    }

    pub fn as_output(&mut self) -> Option<&mut OutputPort> {
        match self {
            Self::Output(port) => Some(port),
            _ => None,
        }
    }

    pub fn close(&mut self) -> io::Result<()> {
        match self {
            Self::Input(port) => port.close(),
            Self::Output(port) => port.close(),
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            Self::Input(port) => port.is_closed(),
            Self::Output(port) => port.is_closed(),
        }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(port) => write!(f, "{}", port),
            Self::Output(port) => write!(f, "{}", port),
        }
    }
}
