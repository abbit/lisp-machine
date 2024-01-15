use std::{io::{ErrorKind, Read, Error}, cell::RefCell, fmt};

pub trait TextInputPort {
    fn ready(&mut self) -> std::io::Result<bool>;
    fn peek(&mut self) -> std::io::Result<char>;
    fn read_one(&mut self) -> std::io::Result<char>;
    fn read_string(&mut self, n: usize) -> std::io::Result<String>;
    fn close(&mut self) -> std::io::Result<()>;
    fn is_closed(&self) -> bool;
}

pub trait OutputPort: std::io::Write {
    fn close(&mut self) -> std::io::Result<()>;
    fn is_closed(&self) -> bool;
}

fn read_u8_helper(reader: &mut impl Read) -> std::io::Result<u8> {
    let mut byte_buf = [0_u8];
    let num_read = reader.read(&mut byte_buf)?;
    if num_read == 0 {
        Err(std::io::Error::from(ErrorKind::UnexpectedEof))
    } else {
        Ok(byte_buf[0])
    }
}

fn read_char_helper(reader: &mut impl Read) -> std::io::Result<char> {
    let mut buf = [0_u8; 4];
    for i in 0..4 {
        let maybe_u8 = read_u8_helper(reader);
        match maybe_u8 {
            Err(e) => {
                return if i != 0 && e.kind() == ErrorKind::UnexpectedEof {
                    Err(std::io::Error::new(
                        ErrorKind::InvalidData,
                        "stream does not contain valid UTF-8",
                    ))
                } else {
                    Err(e)
                }
            }
            Ok(b) => buf[i] = b,
        }
        let uchar = std::char::from_u32(u32::from_le_bytes(buf));
        if let Some(c) = uchar {
            return Ok(c);
        }
    }
    Err(std::io::Error::new(
        ErrorKind::InvalidData,
        "stream does not contain valid UTF-8",
    ))
}

pub struct FileTextInputPort {
    reader: Option<std::io::BufReader<std::fs::File>>,
    peek_buffer: Option<char>,
}

impl FileTextInputPort {
    pub fn new(name: &std::path::Path) -> std::io::Result<Self> {
        let file = std::fs::File::open(name)?;
        Ok(Self {
            reader: Some(std::io::BufReader::new(file)),
            peek_buffer: None,
        })
    }
}

impl TextInputPort for FileTextInputPort {
    fn ready(&mut self) -> std::io::Result<bool> {
        Ok(true)
    }

    fn peek(&mut self) -> std::io::Result<char> {
        if let Some(c) = self.peek_buffer {
            Ok(c)
        } else {
            let c = read_char_helper(self.reader.as_mut().unwrap())?;
            self.peek_buffer = Some(c);
            Ok(c)
        }
    }

    fn read_one(&mut self) -> std::io::Result<char> {
        if let Some(c) = self.peek_buffer {
            self.peek_buffer = None;
            Ok(c)
        } else {
            read_char_helper(self.reader.as_mut().unwrap())
        }
    }

    fn read_string(&mut self, n: usize) -> std::io::Result<String> {
        let mut result = String::with_capacity(n);
        let mut n = n;
        if let Some(c) = self.peek_buffer {
            self.peek_buffer = None;
            n -= 1;
            result.push(c);
        }
        for _ in 0..n {
            match read_char_helper(self.reader.as_mut().unwrap()) {
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        break;
                    }
                }
                Ok(c) => result.push(c),
            }
        }
        if n != 0 && result.is_empty() {
            Err(std::io::Error::from(ErrorKind::UnexpectedEof))
        } else {
            Ok(result)
        }
    }

    fn close(&mut self) -> std::io::Result<()> {
        self.reader = None;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.reader.is_none()
    }
}

pub struct StringOutputPort {
    pub(crate) underlying: String,
}

impl std::io::Write for StringOutputPort {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let as_str = std::str::from_utf8(buf).map_err(|_| Error::from(ErrorKind::InvalidData))?;
        self.underlying.push_str(as_str);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl OutputPort for StringOutputPort {
    fn close(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn is_closed(&self) -> bool {
        false
    }
}

pub enum Port {
    TextInputFile(RefCell<Box<FileTextInputPort>>),
    OutputString(RefCell<StringOutputPort>),
    OutputFile(RefCell<Box<dyn OutputPort>>),
}

impl fmt::Debug for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<port>")
    }
}

impl Clone for Port {
    fn clone(&self) -> Self {
        panic!("trying to clone a port");
    }
}

impl PartialEq for Port {
    fn eq(&self, _other: &Self) -> bool {
        panic!("trying to compare ports");
    }
}