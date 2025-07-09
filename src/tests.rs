use super::*;
use std::io::Cursor;

#[test]
/// This test case is loosely equivalent to running the command:
/// cat tech-test-spalk/test_success.ts | target/debug/spalk-tech-test.exe
fn test_success_case() -> anyhow::Result<()> {
    let test_data = std::fs::read("tech-test-spalk/test_success.ts")?;
    let cursor = Cursor::new(test_data);
    let output = parse_packets(cursor, false)?;
    assert_eq!(output.len(), 53191, "Number of output lines should match");
    Ok(())
}

#[test]
fn test_failure_case() -> anyhow::Result<()> {
    let test_data = std::fs::read("tech-test-spalk/test_failure.ts")?;
    let cursor = Cursor::new(test_data);
    match parse_packets(cursor, false) {
        Err(e) => {
            assert_eq!(
                e.to_string(),
                "No sync byte present in packet 20535, offset 3860580",
                "Output did not match expected results."
            );
        }
        _ => panic!("Unexpected success"),
    }

    Ok(())
}

#[test]
/// This test case is equivalent to receiving a corrupted packet
fn test_incomplete_first_packet() -> anyhow::Result<()> {
    let test_data = std::fs::read("tech-test-spalk/test_success.ts")?;
    let mut cursor = Cursor::new(test_data);
    cursor.set_position(1);
    let output = parse_packets(cursor, false)?;
    assert_eq!(output.len(), 53190, "Number of output lines should match");
    Ok(())
}
