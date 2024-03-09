// SPDX-License-Identifier: MIT OR Apache-2.0

//! Serialize with an added prefix on every field name and deserialize by
//! trimming away the prefix.
//! Author: Jonas Bushart, Marcin Kaźmierczak
//! Source: https://github.com/jonasbb/serde_with/

use std::fmt;

use serde::{
    de::{self, DeserializeSeed, IgnoredAny, IntoDeserializer as _, MapAccess, Visitor},
    forward_to_deserialize_any,
    ser::{self, Impossible, SerializeMap, SerializeStruct},
    Deserializer, Serialize, Serializer,
};

#[macro_export]
macro_rules! with_prefix {
    ($module:ident $prefix:expr) => {$crate::with_prefix!($module $prefix, &[]);};
    ($module:ident $prefix:expr, $ignored_prefixes:expr) => {
        mod $module {
            use serde::{Deserialize, Deserializer, Serialize, Serializer};
            use $crate::serde::WithPrefix;

            #[allow(dead_code)]
            pub fn serialize<T, S>(object: &T, serializer: S) -> serde_with::__private__::Result<S::Ok, S::Error>
            where
                T: Serialize,
                S: Serializer,
            {
                object.serialize(WithPrefix {
                    delegate: serializer,
                    prefix: $prefix,
                    ignored_prefixes: $ignored_prefixes,
                })
            }

            #[allow(dead_code)]
            pub fn deserialize<'de, T, D>(deserializer: D) -> serde_with::__private__::Result<T, D::Error>
            where
                T: Deserialize<'de>,
                D: Deserializer<'de>,
            {
                T::deserialize(WithPrefix {
                    delegate: deserializer,
                    prefix: $prefix,
                    ignored_prefixes: $ignored_prefixes,
                })
            }
        }
    };
}

pub struct WithPrefix<'a, T> {
    pub delegate: T,
    pub prefix: &'a str,
    pub ignored_prefixes: &'a [&'a str],
}

impl<'a, T> Serialize for WithPrefix<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.delegate.serialize(WithPrefix {
            delegate: serializer,
            prefix: self.prefix,
            ignored_prefixes: self.ignored_prefixes,
        })
    }
}

impl<'a, S> Serializer for WithPrefix<'a, S>
where
    S: Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = WithPrefix<'a, S::SerializeMap>;
    type SerializeStruct = WithPrefix<'a, S::SerializeMap>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        for prefix in self.ignored_prefixes {
            if v.starts_with(prefix) {
                return self.delegate.serialize_str(v);
            }
        }

        self.delegate
            .collect_str(&format_args!("{}{}", self.prefix, v))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.delegate.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.delegate.serialize_some(&WithPrefix {
            delegate: value,
            prefix: self.prefix,
            ignored_prefixes: self.ignored_prefixes,
        })
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(WithPrefix {
            delegate: self.delegate.serialize_map(len)?,
            prefix: self.prefix,
            ignored_prefixes: self.ignored_prefixes,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("wrong type for with_prefix"))
    }
}

impl<'a, S> SerializeMap for WithPrefix<'a, S>
where
    S: SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.delegate.serialize_key(&WithPrefix {
            delegate: key,
            prefix: self.prefix,

            ignored_prefixes: self.ignored_prefixes,
        })
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.delegate.serialize_value(value)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        self.delegate.serialize_entry(
            &WithPrefix {
                delegate: key,
                prefix: self.prefix,

                ignored_prefixes: self.ignored_prefixes,
            },
            value,
        )
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.delegate.end()
    }
}

impl<'a, S> SerializeStruct for WithPrefix<'a, S>
where
    S: SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        for prefix in self.ignored_prefixes {
            if key.starts_with(prefix) {
                return self.delegate.serialize_entry(key, value);
            }
        }

        let mut prefixed_key = String::with_capacity(self.prefix.len() + key.len());
        prefixed_key.push_str(self.prefix);
        prefixed_key.push_str(key);
        self.delegate.serialize_entry(&prefixed_key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.delegate.end()
    }
}

impl<'de, 'a, T> DeserializeSeed<'de> for WithPrefix<'a, T>
where
    T: DeserializeSeed<'de>,
{
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.delegate.deserialize(WithPrefix {
            delegate: deserializer,
            prefix: self.prefix,
            ignored_prefixes: self.ignored_prefixes,
        })
    }
}

impl<'de, 'a, D> Deserializer<'de> for WithPrefix<'a, D>
where
    D: Deserializer<'de>,
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_map(WithPrefix {
            delegate: visitor,
            prefix: self.prefix,
            ignored_prefixes: self.ignored_prefixes,
        })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_any(WithPrefixOption {
            first_key: None,
            delegate: visitor,
            prefix: self.prefix,
            ignored_prefixes: self.ignored_prefixes,
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_identifier(WithPrefix {
            delegate: visitor,
            prefix: self.prefix,
            ignored_prefixes: self.ignored_prefixes,
        })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum ignored_any
    }
}

impl<'de, 'a, V> Visitor<'de> for WithPrefix<'a, V>
where
    V: Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.delegate.expecting(formatter)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.delegate.visit_map(WithPrefix {
            delegate: map,
            prefix: self.prefix,

            ignored_prefixes: self.ignored_prefixes,
        })
    }
}

impl<'de, 'a, A> MapAccess<'de> for WithPrefix<'a, A>
where
    A: MapAccess<'de>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        while let Some(s) = self.delegate.next_key::<String>()? {
            for prefix in self.ignored_prefixes {
                if s.starts_with(prefix) {
                    return seed.deserialize(s.into_deserializer()).map(Some);
                }
            }

            if let Some(without_prefix) = s.strip_prefix(self.prefix) {
                return seed
                    .deserialize(without_prefix.into_deserializer())
                    .map(Some);
            }

            self.delegate.next_value::<IgnoredAny>()?;
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.delegate.next_value_seed(seed)
    }
}

pub struct WithPrefixOption<'a, T> {
    first_key: Option<String>,
    delegate: T,
    prefix: &'a str,
    ignored_prefixes: &'a [&'a str],
}

impl<'de, 'a, V> Visitor<'de> for WithPrefixOption<'a, V>
where
    V: Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.delegate.expecting(formatter)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.delegate.visit_none()
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(s) = map.next_key::<String>()? {
            for prefix in self.ignored_prefixes {
                if s.starts_with(prefix) {
                    return self.delegate.visit_some(WithPrefixOption {
                        first_key: Some(s),
                        delegate: map,
                        prefix: self.prefix,

                        ignored_prefixes: self.ignored_prefixes,
                    });
                }
            }

            if s.starts_with(self.prefix) {
                return self.delegate.visit_some(WithPrefixOption {
                    first_key: Some(s),
                    delegate: map,
                    prefix: self.prefix,

                    ignored_prefixes: self.ignored_prefixes,
                });
            }
            map.next_value::<IgnoredAny>()?;
        }
        self.delegate.visit_none()
    }
}

impl<'de, 'a, A> Deserializer<'de> for WithPrefixOption<'a, A>
where
    A: MapAccess<'de>,
{
    type Error = A::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, 'a, A> MapAccess<'de> for WithPrefixOption<'a, A>
where
    A: MapAccess<'de>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(s) = self.first_key.take() {
            for prefix in self.ignored_prefixes {
                if s.starts_with(prefix) {
                    return seed.deserialize(s.into_deserializer()).map(Some);
                }
            }

            let without_prefix = s[self.prefix.len()..].into_deserializer();
            return seed.deserialize(without_prefix).map(Some);
        }
        while let Some(s) = self.delegate.next_key::<String>()? {
            for prefix in self.ignored_prefixes {
                if s.starts_with(prefix) {
                    return seed.deserialize(s.into_deserializer()).map(Some);
                }
            }

            if let Some(without_prefix) = s.strip_prefix(self.prefix) {
                return seed
                    .deserialize(without_prefix.into_deserializer())
                    .map(Some);
            }
            self.delegate.next_value::<IgnoredAny>()?;
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.delegate.next_value_seed(seed)
    }
}
