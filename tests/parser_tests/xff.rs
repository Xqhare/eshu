use athena::XffValue;
use eshu::Store;
use std::collections::BTreeMap;

#[test]
fn test_xff_exists() {
    let store = Store::Exists;
    let xff: XffValue = store.into();
    assert_eq!(xff, XffValue::from(true));
}

#[test]
fn test_xff_value() {
    let store = Store::Value(vec!["hello".to_string(), "world".to_string()]);
    let xff: XffValue = store.into();
    let arr = xff.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0], XffValue::from("hello"));
    assert_eq!(arr[1], XffValue::from("world"));
}

#[test]
fn test_xff_key_value() {
    let mut map = BTreeMap::new();
    map.insert("a".to_string(), "1".to_string());
    map.insert("b".to_string(), "2".to_string());
    let store = Store::KeyValue(map);
    let xff: XffValue = store.into();
    let obj = xff.as_object().unwrap();
    assert_eq!(obj.get("a").unwrap(), &XffValue::from("1"));
    assert_eq!(obj.get("b").unwrap(), &XffValue::from("2"));
}
