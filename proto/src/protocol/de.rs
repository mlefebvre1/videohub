use super::error::{Error, Result};
use serde::{
    de::{self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor},
    forward_to_deserialize_any, Deserialize,
};
use std::ops::{AddAssign, MulAssign};

pub struct Deserializer<'de> {
    input: &'de str,
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer { input: s };
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'de> Deserializer<'de> {
    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn parse_bool(&mut self) -> Result<bool> {
        if self.input.starts_with("true") {
            self.input = &self.input["true".len()..];
            Ok(true)
        } else if self.input.starts_with("false") {
            self.input = &self.input["false".len()..];
            Ok(false)
        } else {
            Err(Error::ExpectedBoolean(self.input.to_string()))
        }
    }

    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8>,
    {
        let mut int = match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int);
                }
            }
        }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        match self.input.find(|c| c == ':' || c == '\n') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len..];
                Ok(s)
            }
            None => {
                // If no delimiter was found take the rest
                let s = self.input;
                let len = s.chars().count();
                self.input = &self.input[len..];
                Ok(s)
            }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Unsupported types end-up here
        Err(Error::UnsupportedType)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 f32 f64 char bytes byte_buf option
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(NewLineSeparated::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SpaceSeparated::new(self))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(NewLineSeparated::new(self))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self.parse_string()?.into_deserializer())
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct NewLineSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> NewLineSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        NewLineSeparated { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for NewLineSeparated<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.peek_char()? == '\n' {
            return Ok(None);
        }

        let result = seed.deserialize(&mut *self.de).map(Some);
        self.de.next_char()?; // consume the '\n' between elements
        result
    }
}

impl<'de, 'a> MapAccess<'de> for NewLineSeparated<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.de.input.is_empty() || self.de.peek_char()? == '\n' {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.de.next_char()? != ':' {
            return Err(Error::ExpectedMapColon);
        }

        self.de.next_char()?; // remove the whitespace between ':' and the value
        let result = seed.deserialize(&mut *self.de);
        self.de.next_char()?; // consume the \n
        result
    }
}

struct SpaceSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SpaceSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        SpaceSeparated { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for SpaceSeparated<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.input.is_empty() || self.de.peek_char()? == '\n' {
            return Ok(None);
        }

        let result = seed.deserialize(&mut *self.de).map(Some);
        if !self.de.input.is_empty() && self.de.peek_char()? == ' ' {
            self.de.next_char()?; // consume the whitespace between elements
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::protocol::*;

    #[test]
    fn test_numbers() {
        assert_eq!(from_str::<usize>("42").unwrap(), 42);
        assert_eq!(from_str::<u32>("196").unwrap(), 196);
    }

    #[test]
    fn test_strings() {
        assert_eq!(from_str::<String>("Foo").unwrap(), "Foo".to_string());
        assert_eq!(from_str::<String>("Foo\n").unwrap(), "Foo".to_string());
        assert_eq!(from_str::<String>("Foo:").unwrap(), "Foo".to_string());
    }

    #[test]
    fn test_seq() {
        let expected = vec![1_u32, 2, 3];
        let s = "1\n2\n3\n\n";
        let result: Vec<u32> = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tuple() {
        let expected = (1_u32, 2_u32, 3_u32, 5_u32);
        let s = "1 2 3 5";
        let result: (u32, u32, u32, u32) = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_struct_tuple() {
        #[derive(Deserialize, Debug, PartialEq, Eq)]
        struct Foo(usize, String);

        let expected = Foo(2, "Bar 2".to_string());
        let s = "2 Bar 2";
        let result: Foo = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_protocol_preamble() {
        let expected = ProtocolPreamble {
            version: "2.3".to_string(),
        };
        let s = "Version: 2.3\n\n";
        let result: ProtocolPreamble = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_device_info() {
        let expected = DeviceInfo {
            device_present: DevicePresent::Present,
            model_name: "Some model name".to_string(),
            friendly_name: "Bar".to_string(),
            unique_id: "XXXX".to_string(),
            nb_video_inputs: 40,
            nb_video_processing_units: 2,
            nb_video_outputs: 40,
            nb_video_monitoring_outputs: 1,
            nb_serial_ports: 0,
        };

        let s = "Device present: true\n\
                       Model name: Some model name\n\
                       Friendly name: Bar\n\
                       Unique ID: XXXX\n\
                       Video inputs: 40\n\
                       Video processing units: 2\n\
                       Video outputs: 40\n\
                       Video monitoring outputs: 1\n\
                       Serial ports: 0\n\n";
        let result: DeviceInfo = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_labels() {
        let expected = vec![Label(2, "Bar 2".to_string()), Label(3, "Foo 3".to_string())];
        let s = "2 Bar 2\n3 Foo 3\n\n";
        let result: Vec<Label> = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_output_locks() {
        let expected = vec![
            OutputLock(2, LockStatus::Locked),
            OutputLock(3, LockStatus::Unlocked),
            OutputLock(39, LockStatus::Owned),
            OutputLock(0, LockStatus::ForceUnlock),
        ];
        let s = "2 L\n3 U\n39 O\n0 F\n\n";
        let result: Vec<OutputLock> = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_output_routings() {
        let expected = vec![
            Route(39, 1),
            Route(15, 13),
            Route(12, 6),
            Route(3, 28),
            Route(97, 45),
        ];
        let s = "39 1\n15 13\n12 6\n3 28\n97 45\n\n";
        let result: Vec<Route> = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_configuration() {
        let expected = Configuration { take_mode: true };
        let s = "Take Mode: true\n\n";
        let result: Configuration = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_end_prelude() {
        let expected = EndPrelude;
        let s = "\n";
        let result: EndPrelude = from_str(s).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hub_info() {
        let expected = HubInfo {
            protocol_preamble: ProtocolPreamble {
                version: "2.3".to_string(),
            },
            device_info: DeviceInfo {
                device_present: DevicePresent::Present,
                model_name: "Some model name".to_string(),
                friendly_name: "Bar".to_string(),
                unique_id: "XXXX".to_string(),
                nb_video_inputs: 40,
                nb_video_processing_units: 2,
                nb_video_outputs: 40,
                nb_video_monitoring_outputs: 1,
                nb_serial_ports: 0,
            },
            input_labels: vec![Label(2, "Bar 2".to_string()), Label(3, "Foo 3".to_string())],
            output_labels: vec![Label(2, "Bar 2".to_string()), Label(3, "Foo 3".to_string())],
            video_output_locks: vec![
                OutputLock(2, LockStatus::Locked),
                OutputLock(3, LockStatus::Unlocked),
                OutputLock(39, LockStatus::Owned),
                OutputLock(0, LockStatus::ForceUnlock),
            ],
            video_output_routing: vec![
                Route(39, 1),
                Route(15, 13),
                Route(12, 6),
                Route(3, 28),
                Route(97, 45),
            ],
            configuration: Configuration { take_mode: true },
            end_prelude: EndPrelude,
        };

        let s = "PROTOCOL PREAMBLE:\n\
                       Version: 2.3\n\
                       \n\
                       VIDEOHUB DEVICE:\n\
                       Device present: true\n\
                       Model name: Some model name\n\
                       Friendly name: Bar\n\
                       Unique ID: XXXX\n\
                       Video inputs: 40\n\
                       Video processing units: 2\n\
                       Video outputs: 40\n\
                       Video monitoring outputs: 1\n\
                       Serial ports: 0\n\
                       \n\
                       INPUT LABELS:\n\
                       2 Bar 2\n\
                       3 Foo 3\n\
                       \n\
                       OUTPUT LABELS:\n\
                       2 Bar 2\n\
                       3 Foo 3\n\
                       \n\
                       VIDEO OUTPUT LOCKS:\n\
                       2 L\n\
                       3 U\n\
                       39 O\n\
                       0 F\n\
                       \n\
                       VIDEO OUTPUT ROUTING:\n\
                       39 1\n\
                       15 13\n\
                       12 6\n\
                       3 28\n\
                       97 45\n\
                       \n\
                       CONFIGURATION:\n\
                       Take Mode: true\n\
                       \n\
                       END PRELUDE:\n\
                       \n";
        let result: HubInfo = from_str(s).unwrap();
        assert_eq!(result, expected);
    }
}
