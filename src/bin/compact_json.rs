use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Data {
	Full(StructFull),
	Compact(StructCompact),
}

#[derive(Serialize, Deserialize)]
struct StructFull {
	a: u8,
	b: u8,
	c: u8,
	d: Option<u8>,
}

#[derive(Serialize, Deserialize)]
struct StructCompact {
	a: u8,
	b: u8,
}
fn check(data: Data) {
	match data {
		Data::Full(_) => println!("Full"),
		Data::Compact(_) => println!("Compact"),
	}
}
fn main() {
	check(serde_json::from_str(r#"{"a": 1, "b": 2, "c": 3, "d": 4}"#).unwrap());	
	check(serde_json::from_str(r#"{"a": 1, "b": 2}"#).unwrap());	
	check(serde_json::from_str(r#"{"a": 1, "b": 2, "c": 2}"#).unwrap());	
	check(serde_json::from_str(r#"{"a": 1, "b": 2, "d": 2}"#).unwrap());	
}