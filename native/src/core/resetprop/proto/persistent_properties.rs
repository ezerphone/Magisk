// Automatically generated rust module for 'persistent_properties.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct PersistentProperties {
    pub properties: Vec<mod_PersistentProperties::PersistentPropertyRecord>,
}

impl<'a> MessageRead<'a> for PersistentProperties {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.properties.push(r.read_message::<mod_PersistentProperties::PersistentPropertyRecord>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PersistentProperties {
    fn get_size(&self) -> usize {
        0
        + self.properties.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.properties { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_PersistentProperties {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct PersistentPropertyRecord {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl<'a> MessageRead<'a> for PersistentPropertyRecord {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = Some(r.read_string(bytes)?.to_owned()),
                Ok(18) => msg.value = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PersistentPropertyRecord {
    fn get_size(&self) -> usize {
        0
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len(m.len()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len(m.len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.name.as_ref().map_or(Ok(()), |m| w.write_with_tag(10, |w| w.write_string(&m)))?;
        self.value.as_ref().map_or(Ok(()), |m| w.write_with_tag(18, |w| w.write_string(&m)))?;
        Ok(())
    }
}

}

