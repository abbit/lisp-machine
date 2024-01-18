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

trait InputPortTrait {
    fn peek(&mut self) -> io::Result<char>;
    fn read_char(&mut self) -> io::Result<char>;
    fn read_string(&mut self) -> io::Result<String>;
    fn close(&mut self) -> io::Result<()>;
    fn is_closed(&self) -> bool;
}

trait OutputPortTrait: Write {
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

#[derive(Debug)]
enum InputPortInner {
    Stdin(PeekableBufReader<Stdin>),
    File(ReaderInputPort<File>),
}

#[derive(Debug)]
/// Input port.
pub struct InputPort(InputPortInner);

impl PartialEq for InputPort {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (InputPortInner::Stdin(_), InputPortInner::Stdin(_)) => true,
            (InputPortInner::File(file1), InputPortInner::File(file2)) => {
                match (file1.get_inderlying(), file2.get_inderlying()) {
                    (Some(file1), Some(file2)) => {
                        #[cfg(unix)]
                        let fd1 = file1.as_raw_fd();
                        #[cfg(windows)]
                        let fd1 = file1.as_raw_handle();
                        #[cfg(unix)]
                        let fd2 = file2.as_raw_fd();
                        #[cfg(windows)]
                        let fd2 = file2.as_raw_handle();
                        fd1 == fd2
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl InputPort {
    /// Creates a new input port that reads from stdin.
    pub fn new_stdin() -> Self {
        Self(InputPortInner::Stdin(PeekableBufReader::new(io::stdin())))
    }

    /// Creates a new input port that reads from a file.
    pub fn new_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self(InputPortInner::File(ReaderInputPort::new(
            File::open(path)?,
        ))))
    }

    /// Returns the next character in the input port without consuming it.
    pub fn peek(&mut self) -> io::Result<char> {
        match &mut self.0 {
            InputPortInner::Stdin(port) => port.peek(),
            InputPortInner::File(port) => port.peek(),
        }
    }

    /// Returns the next character in the input port and consumes it.
    pub fn read_char(&mut self) -> io::Result<char> {
        match &mut self.0 {
            InputPortInner::Stdin(port) => port.read_char(),
            InputPortInner::File(port) => port.read_char(),
        }
    }

    /// Returns the rest of the input port as a string.
    pub fn read_string(&mut self) -> io::Result<String> {
        match &mut self.0 {
            InputPortInner::Stdin(port) => port.read_string(),
            InputPortInner::File(port) => port.read_string(),
        }
    }

    /// Closes the input port.
    pub fn close(&mut self) -> io::Result<()> {
        match &mut self.0 {
            InputPortInner::Stdin(_) => Ok(()),
            InputPortInner::File(port) => port.close(),
        }
    }

    /// Checks if the input port is closed.
    pub fn is_closed(&self) -> bool {
        match &self.0 {
            InputPortInner::Stdin(_) => false,
            InputPortInner::File(port) => port.is_closed(),
        }
    }
}

impl fmt::Display for InputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            InputPortInner::Stdin(_) => write!(f, "#<stdin input port>"),
            InputPortInner::File(port) => match port.get_inderlying() {
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
enum OutputPortInner {
    Stdout(WriterOutputPort<Stdout>),
    File(WriterOutputPort<File>),
}

#[derive(Debug)]
/// Output port.
pub struct OutputPort(OutputPortInner);

impl OutputPort {
    /// Creates a new output port that writes to stdout.
    pub fn new_stdout() -> Self {
        Self(OutputPortInner::Stdout(WriterOutputPort::new(io::stdout())))
    }

    /// Creates a new output port that writes to a file.
    pub fn new_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self(OutputPortInner::File(WriterOutputPort::new(
            File::create(path)?,
        ))))
    }
}

impl PartialEq for OutputPort {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (OutputPortInner::Stdout(_), OutputPortInner::Stdout(_)) => true,
            (OutputPortInner::File(file1), OutputPortInner::File(file2)) => {
                match (file1.get_inderlying(), file2.get_inderlying()) {
                    (Some(file1), Some(file2)) => {
                        #[cfg(unix)]
                        let fd1 = file1.as_raw_fd();
                        #[cfg(windows)]
                        let fd1 = file1.as_raw_handle();
                        #[cfg(unix)]
                        let fd2 = file2.as_raw_fd();
                        #[cfg(windows)]
                        let fd2 = file2.as_raw_handle();
                        fd1 == fd2
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl Write for OutputPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.0 {
            OutputPortInner::Stdout(port) => port.write(buf),
            OutputPortInner::File(port) => port.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match &mut self.0 {
            OutputPortInner::Stdout(port) => port.flush(),
            OutputPortInner::File(port) => port.flush(),
        }
    }
}

impl OutputPort {
    /// Closes the output port.
    pub fn close(&mut self) -> io::Result<()> {
        match &mut self.0 {
            OutputPortInner::Stdout(_) => Ok(()),
            OutputPortInner::File(port) => port.close(),
        }
    }

    /// Checks if the output port is closed.
    pub fn is_closed(&self) -> bool {
        match &self.0 {
            OutputPortInner::Stdout(_) => false,
            OutputPortInner::File(port) => port.is_closed(),
        }
    }
}

impl fmt::Display for OutputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            OutputPortInner::Stdout(_) => write!(f, "#<stdout output port>"),
            OutputPortInner::File(port) => match port.get_inderlying() {
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
/// Port type that can be either input or output.
pub enum Port {
    /// Input port.
    Input(InputPort),
    /// Output port.
    Output(OutputPort),
}

impl Port {
    /// Creates a new input port that reads from stdin.
    pub fn new_stdin() -> Self {
        Self::Input(InputPort::new_stdin())
    }

    /// Creates a new output port that writes to stdout.
    pub fn new_stdout() -> Self {
        Self::Output(OutputPort::new_stdout())
    }

    /// Creates a new input port that reads from a file.
    pub fn new_input_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self::Input(InputPort::new_file(path)?))
    }

    /// Creates a new output port that writes to a file.
    pub fn new_output_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self::Output(OutputPort::new_file(path)?))
    }

    /// Checks if the port is input.
    pub fn is_input(&self) -> bool {
        matches!(self, Self::Input(_))
    }

    /// Checks if the port is output.
    pub fn is_output(&self) -> bool {
        matches!(self, Self::Output(_))
    }

    /// Returns mutable reference to the underlying input port.
    pub fn as_input(&mut self) -> Option<&mut InputPort> {
        match self {
            Self::Input(port) => Some(port),
            _ => None,
        }
    }

    /// Returns mutable reference to the underlying output port.
    pub fn as_output(&mut self) -> Option<&mut OutputPort> {
        match self {
            Self::Output(port) => Some(port),
            _ => None,
        }
    }

    /// Closes the port.
    pub fn close(&mut self) -> io::Result<()> {
        match self {
            Self::Input(port) => port.close(),
            Self::Output(port) => port.close(),
        }
    }

    /// Checks if the port is closed.
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
