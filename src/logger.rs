use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::io::Read;

pub trait Logger {
    fn process(&self, mut stream: Box<Read>) {
        self.set_buffers(vec![], vec![]);

        loop {
            // 8 byte header [ STREAM_TYPE, 0, 0, 0, SIZE1, SIZE2, SIZE3, SIZE4 ]
            let mut header = [0; 8];
            match stream.read_exact(&mut header) {
                Ok(_) => {
                    let payload_size: Vec<u8> = header[4..8].to_vec();
                    let mut buffer = vec![
                        0;
                        Cursor::new(&payload_size)
                            .read_u32::<BigEndian>()
                            .unwrap() as
                            usize
                    ];
                    match stream.read_exact(&mut buffer) {
                        Ok(_) => {
                            match header[0] {
                                // stdin, unhandled
                                0 => break,
                                // stdout
                                // 1 => stdout.append(&mut buffer),
                                1 => self.append_stdout(&mut buffer),
                                // stderr
                                // 2 => stderr.append(&mut buffer),
                                2 => self.append_stderr(&mut buffer),
                                //unhandled
                                _ => break,
                            }
                        }
                        Err(_) => break,
                    };
                }
                Err(_) => break,
            }
        }
    }
    fn set_buffers(&self, stdout: Vec<u8>, stderr: Vec<u8>);
    fn append_stdout(&self, buffer: &mut [u8]);
    fn append_stderr(&self, buffer: &mut [u8]);
}

pub struct FakeLogger;

impl Logger for FakeLogger {
    fn set_buffers(&self, _stdout: Vec<u8>, _stderr: Vec<u8>) {
        //Do Nothing
    }
    fn append_stdout(&self, buffer: &mut [u8]) {
        info!("STDOUT: {}", String::from_utf8_lossy(buffer).to_string());
    }
    fn append_stderr(&self, buffer: &mut [u8]) {
        info!("STDERR: {}", String::from_utf8_lossy(buffer).to_string());
    }
}
