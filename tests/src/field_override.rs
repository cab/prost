include!(concat!(env!("OUT_DIR"), "/field_override.rs"));

#[test]
fn test_field_override_type() {
    let _ = BaseMessage {
        msg: Some(OverriddenMessage {}),
    };
}
