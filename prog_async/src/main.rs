extern crate tokio;
extern crate tokio_io;
extern crate tokio_codec;
#[macro_use] extern crate futures;
extern crate bytes;
extern crate byteorder;
extern crate crc;
extern crate time;
extern crate rand;
extern crate openssl;
extern crate vail_types;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;



use tokio::prelude::*;
//use tokio::timer::Delay;
use tokio::net::TcpStream;
use tokio_codec::{Decoder, Encoder};

//use tokio_io::codec::{Decoder, Encoder};

//use std::time::{Duration, Instant};

use std::io::{Read, Write, Cursor, Result};

//use bytes::{Buf, BufMut, BytesMut, Bytes, IntoBuf};
use bytes::Bytes;

//use byteorder::{ByteOrder, ReadBytesExt, LE, BE};
use byteorder::{ByteOrder, LE};

//use crc::{Hasher32};
//use crc::crc32::{self, Digest};


use vail_types::{types, methods};
use vail_types::serialize::*;
use vail_types::deserialize::*;


mod tcp_full;
use tcp_full::TcpFullCodec;

mod math;
mod rsa;




pub trait TransportMode where Self: Decoder + Encoder {
    fn new() -> Self;
}

pub struct MtprotoClient<T: TransportMode> {
    transport: tokio_codec::Framed<tokio::net::TcpStream, T>,
    time_offset: i64,
}


impl<T: TransportMode> MtprotoClient<T> {
    pub fn new() -> Self {
        MtprotoClient {
            transport: T::new().unwrap(),
            time_offset: 0,
        }
    }

    fn get_msg_id(&self) -> i64 {
        let current_time = time::get_time();

        return (current_time.sec + self.time_offset) << 32 | (current_time.nsec & 0xffff_fffc) as i64;
    }

    pub fn send_unencrypted(&mut self, bytes: &[u8]) -> Result<()> {
        //println!("WRITE UNENCRYPTED BEGIN");
        let auth_key_id: i64 = 0;
        let message_id: i64 = self.get_msg_id();
        let message_data_length: i32 = bytes.len() as i32;

        let mut buffer = vec![0; 8 + 8 + 4 + bytes.len()];
        LE::write_i64(&mut buffer[0..8], auth_key_id);
        LE::write_i64(&mut buffer[8..16], message_id);
        LE::write_i32(&mut buffer[16..20], message_data_length);
        (&mut buffer[20..]).write_all(&bytes)?;

        //println!("\nBYTES + MTPROTO HEADERS:{}", dump_bytes(buffer.as_ref()).unwrap());
        self.transport.send(buffer.as_ref())
            //println!("WRITE UNENCRYPTED END");
    }

    pub fn send_obj_unencrypted<O: Serializable>(&mut self, obj: &O) -> Result<()> {
        let mut buf = Vec::new();
        buf.serialize(obj)?;

        //println!("SEND_OBJ:{}", dump_bytes(&buf).unwrap());
        self.send_unencrypted(&buf)
    }

    pub fn invoke_unencrypted<M: methods::Method>(&mut self, obj: &M) -> Result<M::ReturnType> {
        self.send_obj_unencrypted(obj)?;
        self.recv_obj_unencrypted()
    }

    pub fn recv_unencrypted(&mut self) -> Result<Vec<u8>> {
        //println!("READ UNENCRYPTED BEGIN");
        let buffer = self.transport.recv()?;

        let auth_key_id: i64 = LE::read_i64(&buffer[0..8]);
        println!("auth_key_id: {}", auth_key_id);
        let message_id: i64 = LE::read_i64(&buffer[8..16]);
        println!("message_id: {}", message_id);
        let message_data_length: i32 = LE::read_i32(&buffer[16..20]);

        //println!("READ UNENCRYPTED END");
        Ok(buffer[20..20+message_data_length as usize].to_vec())
    }

    pub fn recv_obj_unencrypted<O: Deserializable>(&mut self) -> Result<O> {
        let res = self.recv_unencrypted()?;
        //println!("RECV_OBJ:{}", dump_bytes(&res).unwrap());

        let mut cur = Cursor::new(res);

        cur.deserialize()
    }
}



fn main() {
    let addr = "149.154.167.40:443".parse().unwrap();

    let task = TcpStream::connect(&addr).and_then(|sock| {
        let framed_sock = TcpFullCodec::new().framed(sock);
        
        let () = framed_sock;
        
        //let data = Bytes::from(&b"Hello world"[..]);
        //framed_sock.send(data)
        
        /*
        framed_sock.for_each(|packet| {
            // do packet
            Ok(())
        })
        */
    });


    //tokio::run(task);
}
