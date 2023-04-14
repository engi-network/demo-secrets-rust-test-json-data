use demo_secrets::{add_two, say_thing};
use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::Read,
    path::PathBuf,
};


const TEST_DATA_FILENAME: &str = "test-data.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestData {
    pub test_twos: Vec<(i32, i32)>,
    pub test_say_thing: Vec<(String, String)>,
}

fn unpack_tests() -> TestData {
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("No cargo manifest");
    let mut path = PathBuf::new();

    path.push(root);
    path.push(TEST_DATA_FILENAME);

    if path.exists() {
        let mut f = File::open(path).expect("Could not open test dataset!");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("File read error");

        serde_json::from_str(&s).expect("Failed to deserialize test data")
    } else {
        panic!("No test dataset present!")
    }
}

#[test]
fn test_twos() {
    let TestData { mut test_twos, .. } = unpack_tests();

    for (index, (input, output)) in test_twos.drain(..).enumerate() {
        assert_eq!(add_two(input), output, "test {} failed with input {:?}: {:?}", index, input, output);
    }
}

#[test]
fn test_say_thing() {
    let TestData { mut test_say_thing, .. } = unpack_tests();

    for (index, (input, output)) in test_say_thing.drain(..).enumerate() {
        assert_eq!(say_thing(&input), output, "test {} failed with input {:?}: {:?}", index, input, output);
    }
}
