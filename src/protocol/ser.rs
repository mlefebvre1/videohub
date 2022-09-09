use serde::{ser, Serialize};

use super::error::{Error, Result};

pub struct Serializer {
    output: String,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += v;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        self.output += name;
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += name;
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += variant;
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.output += name;
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.output += name;
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += name;
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.output += " ";
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.pop();
        self.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.output += " ";
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.pop();
        self.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.output += " ";
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.pop();
        self.output += "\n";

        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += ": ";
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        self.output += ": ";
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        self.output += ": ";
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::protocol::*;

    #[test]
    fn test_protocol_preamble() {
        let preamble = BlockType::ProtocolPreamble(ProtocolPreamble {
            version: "2.3".to_string(),
        });
        assert_eq!(
            &to_string(&preamble).unwrap(),
            "PROTOCOL PREAMBLE:\nVersion: 2.3\n\n"
        );
    }

    #[test]
    fn test_device_info() {
        let device_info = BlockType::DeviceInfo(DeviceInfo {
            device_present: DevicePresent::Present,
            model_name: "Foo".to_string(),
            friendly_name: "Bar".to_string(),
            unique_id: "XXXX".to_string(),
            nb_video_inputs: 40,
            nb_video_processing_units: 2,
            nb_video_outputs: 40,
            nb_video_monitoring_outputs: 1,
            nb_serial_ports: 0,
        });
        let result = to_string(&device_info).unwrap();
        let expected = "VIDEOHUB DEVICE:\n\
                              Device present: true\n\
                              Model name: Foo\n\
                              Friendly name: Bar\n\
                              Unique ID: XXXX\n\
                              Video inputs: 40\n\
                              Video processing units: 2\n\
                              Video outputs: 40\n\
                              Video monitoring outputs: 1\n\
                              Serial ports: 0\n\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_input_labels() {
        let labels = BlockType::InputLabels(vec![
            Label(2, "Bar 2".to_string()),
            Label(3, "Foo 3".to_string()),
        ]);
        let result = to_string(&labels).unwrap();
        assert_eq!(&result, "INPUT LABELS:\n2 Bar 2\n3 Foo 3\n\n");
    }

    #[test]
    fn test_output_labels() {
        let labels = BlockType::OutputLabels(vec![
            Label(2, "Bar 2".to_string()),
            Label(3, "Foo 3".to_string()),
        ]);
        let result = to_string(&labels).unwrap();
        assert_eq!(&result, "OUTPUT LABELS:\n2 Bar 2\n3 Foo 3\n\n");
    }

    #[test]
    fn test_output_locks() {
        let labels = BlockType::VideoOutputLocks(vec![
            OutputLock(30, LockStatus::Locked),
            OutputLock(24, LockStatus::Unlocked),
        ]);
        let result = to_string(&labels).unwrap();
        assert_eq!(&result, "VIDEO OUTPUT LOCKS:\n30 L\n24 U\n\n");
    }

    #[test]
    fn test_output_routing() {
        let labels = BlockType::VideoOutputRouting(vec![Route(0, 5), Route(36, 6), Route(13, 13)]);
        let result = to_string(&labels).unwrap();
        assert_eq!(&result, "VIDEO OUTPUT ROUTING:\n0 5\n36 6\n13 13\n\n");
    }

    #[test]
    fn test_configuration() {
        let config_true = BlockType::Configuration(Configuration { take_mode: true });
        assert_eq!(
            &to_string(&config_true).unwrap(),
            "CONFIGURATION:\nTake Mode: true\n\n"
        );
    }

    #[test]
    fn test_end_prelude() {
        let end = BlockType::EndPrelude(EndPrelude);
        assert_eq!(&to_string(&end).unwrap(), "END PRELUDE:\n\n");
    }

    #[test]
    fn test_enum() {
        let lock_status = LockStatus::Unlocked;
        let result = to_string(&lock_status).unwrap();
        assert_eq!(result, "U");
    }
}
