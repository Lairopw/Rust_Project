use std::{collections::HashMap, io::Write};

use anyhow::Ok;
use byteorder::{LittleEndian, WriteBytesExt};
use uuid::Uuid;

use crate::messages::{
  AuthMessage, ClientId, ClientMessage, ClientPollReply, ClientQuery, ClientReply, Sequence, ServerId, ServerMessage,
};

pub fn u128<W>(w: &mut W, m: &u128) -> anyhow::Result<()>
where
  W: Write,
{
  let m = *m;

  if m < 251 {
    w.write_u8(m as u8)?;
  } 
  else if m <= (u16::MAX as u128) {
    w.write_u8(251)?;
    w.write_u16::<LittleEndian>(m as u16)?;
  }
  else if m <= (u32::MAX as u128) {
    w.write_u8(252)?;
    w.write_u32::<LittleEndian>(m as u32)?;
  }
  else if m <= (u64::MAX as u128) {
    w.write_u8(253)?;
    w.write_u64::<LittleEndian>(m as u64)?;
  } 
  else {
      w.write_u8(254)?;
      w.write_u128::<LittleEndian>(m)?; 
  }
  Ok(())
}


fn uuid<W>(w: &mut W, m: &Uuid) -> anyhow::Result<()>
where
  W: Write,
{

  w.write_all(m.as_bytes())?;
  Ok(())
}

pub fn clientid<W>(w: &mut W, m: &ClientId) -> anyhow::Result<()>
where
  W: Write,
{
  uuid(w, &m.0)?;
  Ok(())
}

pub fn serverid<W>(w: &mut W, m: &ServerId) -> anyhow::Result<()>
where
  W: Write,
{
  uuid(w, &m.0)?;
  Ok(())
}

pub fn string<W>(w: &mut W, m: &str) -> anyhow::Result<()>
where
  W: Write,
{
  let bts = m.as_bytes();
  let sz = bts.len();
  u128(w, &(sz as u128))?;
  w.write_all(bts);
  Ok(())

}

pub fn auth<W>(w: &mut W, m: &AuthMessage) -> anyhow::Result<()>
where
  W: Write,
{
  match m {
      AuthMessage::Hello { user, nonce } => {
        w.write_u8(0)?;
        clientid(w, user)?;
        w.write_all(nonce)?;
      }
      AuthMessage::Nonce { server, nonce } => {
        w.write_u8(1)?;
        serverid(w, server)?;
        w.write_all(nonce)?;
      },
      AuthMessage::Auth { response } => {
        w.write_u8(2)?;
        w.write_all(response)?;
      },  
  }
  Ok(())
}

pub fn server<W>(w: &mut W, m: &ServerMessage) -> anyhow::Result<()>
where
  W: Write,
{
  let message = ServerMessage(m.clone());
  serde_json::to_writer(w, &message)?;
  w.flush()?;
  Ok(())
}

pub fn client<W>(w: &mut W, m: &ClientMessage) -> anyhow::Result<()>
where
  W: Write,
{
  let message = Message::Client(m.clone());
  serde_json::to_writer(w, &message)?;
  w.flush()?;
  Ok(())}

pub fn client_replies<W>(w: &mut W, m: &[ClientReply]) -> anyhow::Result<()>
where
  W: Write,
{
  let message = Message::ClientReplies(m.to_vec());
  serde_json::to_writer(w, &message)?;
  w.flush()?;
  Ok(())
}

pub fn client_poll_reply<W>(w: &mut W, m: &ClientPollReply) -> anyhow::Result<()>
where
  W: Write,
{
  let message = Message::ClientPollReply(m.clone());
  serde_json::to_writer(w, &message)?;
  w.flush()?;
  Ok(())
}

pub fn userlist<W>(w: &mut W, m: &HashMap<ClientId, String>) -> anyhow::Result<()>
where
  W: Write,
{
  let message = Message::UserList(m.clone());
  serde_json::to_writer(w, &message)?;
  w.flush()?;
  Ok(())
}

pub fn client_query<W>(w: &mut W, m: &ClientQuery) -> anyhow::Result<()>
where
  W: Write,
{
  let message = Message::ClientQuery(m.clone());
  serde_json::to_writer(w, &message)?;
  w.flush()?;
  Ok(())
}

pub fn sequence<X, W, ENC>(w: &mut W, m: &Sequence<X>, f: ENC) -> anyhow::Result<()>
where
  W: Write,
  X: serde::Serialize,
  ENC: FnOnce(&mut W, &X) -> anyhow::Result<()>,
{
  todo!()
}
