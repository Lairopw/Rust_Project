use std::{collections::HashMap, io::Read, u128, vec, char::MAX};

use anyhow::Ok;
use byteorder::{LittleEndian, ReadBytesExt};
use uuid::Uuid;

use crate::messages::{
  AuthMessage, ClientId, ClientMessage, ClientPollReply, ClientQuery, ClientReply, Sequence,
  ServerId, ServerMessage,
};

pub fn u128<R: Read>(rd: &mut R) -> anyhow::Result<u128> {
  let first_byte = rd.read_u8()?;
  let n = if first_byte < 251 {
    first_byte as u128
  }
  else if first_byte == 251 {
    rd.read_u16::<LittleEndian>()? as u128
  }
  else if first_byte == 252 {
    rd.read_u32::<LittleEndian>()? as u128
  }
  else if first_byte == 253 {
    rd.read_u64::<LittleEndian>()? as u128
  }
  else if first_byte == 254 {
    rd.read_u128::<LittleEndian>()? as u128
  }
  else{
  anyhow::bail!("should not support")
  };
  Ok(n)
}

fn uuid<R: Read>(rd: &mut R) -> anyhow::Result<Uuid> {
  let sz = rd.read_u8()?;
  if sz != 16{
    anyhow::bail!("invalid size for uuid ! ")
  }
  let mut buf = [0; 16];
  rd.read_exact(&mut buf)?;

  Ok(Uuid::from_bytes(buf))
}

//hint: reuse uuid
pub fn clientid<R: Read>(rd: &mut R) -> anyhow::Result<ClientId> {
  uuid(rd).map(ClientId)
}

//hint: reuse uuid
pub fn serverid<R: Read>(rd: &mut R) -> anyhow::Result<ServerId> {
  uuid(rd).map(ServerId)
}

pub fn string<R: Read>(rd: &mut R) -> anyhow::Result<String> {
  let sz128 = u128(rd)?;
  if (sz128 > usize::MAX as u128 ){
    anyhow::bail!("string too large")
  }
  let sz = u128(rd)? as usize;
  let mut v = vec![0;sz];
  rd.read_exact(&mut v);
  let s = String::from_utf8(v)?;
  Ok(s)
}

pub fn auth<R: Read>(rd: &mut R) -> anyhow::Result<AuthMessage> {
  let tag = rd.read_u8()?;
  let m = match tag {
    0 => {
        let user = clientid(rd)?;
        let nonce_uuid = uuid(rd)?;
        let mut nonce = [0u8; 8];
        nonce.copy_from_slice(&nonce_uuid.as_bytes()[0..8]);
        AuthMessage::Hello { user, nonce }
    },
    1 => {
        let server = serverid(rd)?;
        let nonce_uuid = uuid(rd)?;
        let mut nonce = [0u8; 8];
        nonce.copy_from_slice(&nonce_uuid.as_bytes()[0..8]);
        AuthMessage::Nonce { server, nonce }
    },
    2 => {
        // let response = rd.read_to_end(&mut Vec::new())?;
        // let response_array: [u8; 16] = response[..].try_into().map_err(|_| anyhow::anyhow!("Error: Could not convert vector to array"))?;
        // AuthMessage::Auth { response: response_array }
        let mut response = [0u8;16];
        rd.read_exact(&mut response)?;
        AuthMessage::Auth { response }
    },
    _ => anyhow::bail!(":'(")
    };
  Ok(m)

  // let tag = rd.read_u8()?;
  // let m = match tag { 0 => {
  //   let user = clientid(rd)?;
  //   let mut nonce = uuid(rd)?.as_bytes();
  //   // AuthMessage::Hello { user, nonce }
  //   let mut nonce_bytes = [0u8; 8];
  //   nonce_bytes.copy_from_slice(&nonce[0..8]);
  //   AuthMessage::Hello { user, nonce: nonce_bytes }

  // },
  // 1 => {
  //   let server = serverid(rd)?;
  //   let mut nonce = uuid(rd)?.as_bytes();
  //   let mut nonce_bytes = [0u8; 8];
  //   nonce_bytes.copy_from_slice(&nonce[0..8]);
  //   AuthMessage::Nonce { server, nonce: nonce_bytes }
  //   //AuthMessage::Nonce { server, nonce }
  // },
  // 2 => {
  //   let mut response = [0u8;16];
  //   rd.read_exact(&mut response)?;
  //   AuthMessage::Auth { response }
  //   // let response = rd.read_to_end(&mut Vec::new())?;
  //   // let response_array: [u8; 16] = response[..16].try_into().map_err(|_| anyhow::anyhow!("Error: Could not convert vector to array"))?;
  //   // AuthMessage::Auth { response: response_array }
  // },
  // _ => anyhow::bail!(":'(")
  // };
  // Ok(m)
}

pub fn client<R: Read>(rd: &mut R) -> anyhow::Result<ClientMessage> {
  todo!()
}

pub fn client_replies<R: Read>(rd: &mut R) -> anyhow::Result<Vec<ClientReply>> {
  todo!()
}

pub fn client_poll_reply<R: Read>(rd: &mut R) -> anyhow::Result<ClientPollReply> {
  todo!()
}

pub fn server<R: Read>(rd: &mut R) -> anyhow::Result<ServerMessage> {
  todo!()
}

pub fn userlist<R: Read>(rd: &mut R) -> anyhow::Result<HashMap<ClientId, String>> {
  todo!()
}

pub fn client_query<R: Read>(rd: &mut R) -> anyhow::Result<ClientQuery> {
  todo!()
}

pub fn sequence<X, R: Read, DEC>(rd: &mut R, d: DEC) -> anyhow::Result<Sequence<X>>
where
  DEC: FnOnce(&mut R) -> anyhow::Result<X>,
{
  todo!()
}
