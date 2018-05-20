use byteorder::{LittleEndian, WriteBytesExt};

use std::io::{Cursor, Write};
use std::io;

pub trait Serialize<T> {
    fn serialize(&mut self, obj: T, boxed: bool) -> Result<(), io::Error>;
}

pub trait SerializeVector<'a, T> {
    fn serialize_vec(&mut self, obj: &'a [T], boxed: bool, vec_boxed: bool) -> Result<(), io::Error>;
}

impl<'a, T: 'a> SerializeVector<'a, T> for Cursor<Vec<u8>> where Cursor<Vec<u8>>: Serialize<&'a T> {
    fn serialize_vec(&mut self, vector: &'a [T], boxed: bool, vec_boxed: bool) -> Result<(), io::Error> {
        if vec_boxed {
            Serialize::<&u32>::serialize(self, &0xd8292816_u32, false)?;
        }

        for element in vector {
            Serialize::<&T>::serialize(self, element, boxed)?;
        }

        Ok(())
    }
}

impl<'a> Serialize<&'a i32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i32, _: bool) -> Result<(), io::Error> {
        self.write_i32::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a u32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &u32, _: bool) -> Result<(), io::Error> {
        self.write_u32::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a i64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i64, _: bool) -> Result<(), io::Error> {
        self.write_i64::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a f64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f64, _: bool) -> Result<(), io::Error> {
        self.write_f64::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a [u8; 16]> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &[u8; 16], _: bool) -> Result<(), io::Error> {
        self.write(obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a [u8; 32]> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &[u8; 32], _: bool) -> Result<(), io::Error> {
        self.write(obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a [u8]> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &[u8], _: bool) -> Result<(), io::Error> {
        let mut len = obj.len();

        if len < 254 {
            self.write(&[len as u8])?;
            len += 1;
        } else {
            self.write_all(&[254, len as u8, (len >> 8) as u8, (len >> 16) as u8])?; // 3 bytes
        }

        self.write_all(obj)?;

        for _ in 0..(4 - (len % 4)) % 4 {
            self.write(&[00u8])?;
        }

        Ok(())
    }
}


/*
#[derive(Debug)]
struct AccountTmpPassword {
    tmp_password: Vec<u8>,
    valid_until: i32,
}
*/
// account.tmpPassword#db64fd34 tmp_password:bytes valid_until:int = account.TmpPassword;


/*
inputUserEmpty#b98886cf = InputUser;
inputUserSelf#f7c1b13f = InputUser;
inputUser#d8292816 user_id:int access_hash:long = InputUser;

#[derive(Debug)]
pub enum InputUser {
    User {
        user_id: i32,
        access_hash: i64,
    },

    Self_,

    Empty,
}
*/

/*

use types::InputUser;

impl<'a> Serialize<&'a InputUser> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &InputUser, _: bool) -> Result<(), io::Error> {
        match *obj {
            InputUser::User {
                user_id,
                access_hash,
            } => {
                self.serialize(0xd8292816_u32)?;
                self.serialize(user_id)?;
                self.serialize(access_hash)?;
            }

            InputUser::Self_ => {
                self.serialize(0xf7c1b13f_u32)?;
            }

            InputUser::Empty => {
                self.serialize(0xb98886cf_u32)?;
            }
        };

        Ok(())
    }
}

*/

/*
inputChatPhotoEmpty#1ca48f57 = InputChatPhoto;
inputChatUploadedPhoto#927c55b4 file:InputFile = InputChatPhoto;
inputChatPhoto#8953ad37 id:InputPhoto = InputChatPhoto;

#[derive(Debug)]
pub enum InputChatPhoto {
    Photo {
        id: Box<InputPhoto>,
    },

    Empty,

    Uploaded {
        file: Box<InputFile>,
    },
}
*/

/*
use types::InputChatPhoto;

impl<'a> Serialize<&'a InputChatPhoto> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &InputChatPhoto, _: bool) -> Result<(), io::Error> {
        match *obj {
            InputChatPhoto::Photo {
                ref id
            } => {
                self.serialize(0x8953ad37_u32)?;
                self.serialize(id.as_ref())?;
            }

            InputChatPhoto::Empty => {
                self.serialize(0x1ca48f57_u32)?;
            }

            InputChatPhoto::Uploaded {
                ref file
            } => {
                self.serialize(0x927c55b4_u32)?;
                self.serialize(file.as_ref())?;
            }
        };

        Ok(())
    }
}
*/

/*
inputPhotoEmpty#1cd7bf0d = InputPhoto;
inputPhoto#fb95c6c4 id:long access_hash:long = InputPhoto;

#[derive(Debug)]
pub enum InputPhoto {
    Photo {
        id: i64,
        access_hash: i64,
    },

    Empty,
}
*/

/*
use types::InputPhoto;

impl<'a> Serialize<&'a InputPhoto> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &InputPhoto, _: bool) -> Result<(), io::Error> {
        match *obj {
            InputPhoto::Photo { id, access_hash } => {
                self.serialize(0xfb95c6c4_u32)?;
                self.serialize(id)?;
                self.serialize(access_hash)?;
            }

            InputPhoto::Empty => {
                self.serialize(0x1cd7bf0d_u32)?;
            }
        };

        Ok(())
    }
}

*/

/*
inputFile#f52ff27f id:long parts:int name:string md5_checksum:string = InputFile;
inputFileBig#fa4f0bb5 id:long parts:int name:string = InputFile;

#[derive(Debug)]
pub enum InputFile {
    Big {
        id: i64,
        parts: i32,
        name: String,
    },

    File {
        id: i64,
        parts: i32,
        name: String,
        md5_checksum: String,
    },
}
*/
/*
use types::InputFile;

impl<'a> Serialize<&'a InputFile> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &InputFile, _: bool) -> Result<(), io::Error> {
        match *obj {
            InputFile::Big {
                id,
                parts,
                ref name,
            } => {
                self.serialize(0xd8292816_u32)?;
                self.serialize(id)?;
                self.serialize(parts)?;
                self.serialize(name)?;
            }

            InputFile::File {
                id,
                parts,
                ref name,
                ref md5_checksum,
            } => {
                self.serialize(0xd8292816_u32)?;
                self.serialize(id)?;
                self.serialize(parts)?;
                self.serialize(name)?;
                self.serialize(md5_checksum)?;
            }
        };

        Ok(())
    }
}
*/
