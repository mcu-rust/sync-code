use std::fs;
use sync_code;

#[test]
fn test_sync_code() {
    sync_code::Builder::new()
        .add("tests/data1/target.txt", "tests/data1/source.txt")
        .add("tests/data2/target.txt", "tests/data2/source.txt")
        .sync();

    assert_eq!(
        fs::read_to_string("tests/data1/target.txt").unwrap(),
        fs::read_to_string("tests/data1/source.txt").unwrap()
    );
    assert_eq!(
        fs::read_to_string("tests/data2/target.txt").unwrap(),
        fs::read_to_string("tests/data2/expected.txt").unwrap()
    );
}
