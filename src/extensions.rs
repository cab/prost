pub trait Extension {
    type Value: crate::Message + Default;
    const NUMBER: u32;

    fn decode(tag: u32, values: &[impl AsRef<[u8]>]) -> Option<Self::Value> {
        use crate::Message;

        if tag != Self::NUMBER {
            return None;
        }

        let mut message = Self::Value::decode_length_delimited::<&[u8]>(values[0].as_ref()).ok()?;
        for bytes in &values[1..] {
            message.merge_length_delimited(bytes.as_ref()).ok()?;
        }
        Some(message)
    }
}
