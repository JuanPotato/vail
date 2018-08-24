use bytes::{BufMut, BytesMut, Bytes};
use byteorder::{ByteOrder, LE};
use crc::{Hasher32, crc32};
use std::io;

use tokio_codec::{Decoder, Encoder};

use TransportMode;

#[derive(Debug)]
pub struct TcpFullCodec {
    state: TcpFullState,
    seq_no: i32,
}

#[derive(Debug)]
pub enum TcpFullState {
    Length,
    Data(usize),
}

impl TcpFullCodec {
    fn decode_len(&mut self, src: &mut BytesMut) -> io::Result<Option<usize>> {
        if src.len() < 4 {
            // Not enough data
            return Ok(None);
        }

        let len_bytes = src.split_to(4);
        let len = LE::read_i32(&len_bytes);

        if len < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Provided length was negative."));
        }

        let n = len as usize - 4; // remove 4 to account for the bytes used by the length

        // Ensure that the buffer has enough space to read the incoming payload
        src.reserve(n);

        return Ok(Some(n));
    }

    fn decode_data(&self, n: usize, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        // At this point, the buffer has already had the required capacity
        // reserved. All there is to do is read.
        if src.len() < n {
            return Ok(None);
        }

        let seq_no_bytes = src.split_to(4);
        let seq_no = LE::read_i32(&seq_no_bytes);

        let data = src.split_to(n - 8);

        let checksum_bytes = src.split_to(4);
        let given_checksum = LE::read_u32(&checksum_bytes);

        let mut digest = crc32::Digest::new(crc32::IEEE);
        digest_i32_le(&mut digest, n as i32 + 4); // length
        digest.write(&seq_no_bytes); // seq_no
        digest.write(&data);
        let calc_cecksum = digest.sum32();

        if given_checksum != calc_cecksum {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Checksums do not match."));
        }

        if seq_no != self.seq_no + 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect sequence number."));
        }

        Ok(Some(data))
    }
}

impl Decoder for TcpFullCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        let n = match self.state {
            TcpFullState::Length => {
                match try!(self.decode_len(src)) {
                    Some(n) => {
                        self.state = TcpFullState::Data(n);
                        n
                    }
                    None => return Ok(None),
                }
            }
            TcpFullState::Data(n) => n,
        };

        match try!(self.decode_data(n, src)) {
            Some(data) => {
                // Update the decode state
                self.state = TcpFullState::Length;

                // Make sure the buffer has enough space to read the next head
                src.reserve(4);

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}

impl Encoder for TcpFullCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn encode(&mut self, data: Bytes, buf: &mut BytesMut) -> Result<(), io::Error> {
        let length = data.len() + 4 * 3;
        buf.reserve(length);

        let mut digest = crc32::Digest::new(crc32::IEEE);
        digest_i32_le(&mut digest, length as i32); 
        digest_i32_le(&mut digest, self.seq_no);
        digest.write(&data);
        let checksum = digest.sum32();

        buf.put_i32_le(length as i32);
        buf.put_i32_le(self.seq_no);
        buf.extend_from_slice(&data);
        buf.put_u32_le(checksum);
        
        Ok(())
    }
}

impl TransportMode for TcpFullCodec {
    fn new() -> Self {
        TcpFullCodec {
            state: TcpFullState::Length,
            seq_no: 0,
        }
    }
}

fn digest_i32_le(digest: &mut crc32::Digest, n: i32) {
    let mut buf = [0; 4];
    LE::write_i32(&mut buf, n);
    digest.write(&buf);
}

