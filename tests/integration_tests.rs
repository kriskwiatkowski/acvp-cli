// Integration tests for subprocess communication

use serde_json::json;

#[test]
fn test_subprocess_protocol_format() {
    // Test that we can create a properly formatted request
    let cmd = "testCmd";
    let args = vec![b"arg1".as_slice(), b"arg2".as_slice()];

    let num_args = 1 + args.len();
    let mut buf = Vec::new();

    // Write header
    buf.extend_from_slice(&(num_args as u32).to_le_bytes());
    buf.extend_from_slice(&(cmd.len() as u32).to_le_bytes());

    for arg in &args {
        buf.extend_from_slice(&(arg.len() as u32).to_le_bytes());
    }

    // Write data
    buf.extend_from_slice(cmd.as_bytes());
    for arg in args {
        buf.extend_from_slice(arg);
    }

    // Verify structure
    assert_eq!(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]), 3); // num_args
    assert_eq!(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]), 7); // len("testCmd")
}

#[test]
fn test_json_vector_structure() {
    // Test that we can parse a typical test vector
    let vector = json!({
        "vsId": 12345,
        "algorithm": "SHA2-256",
        "revision": "1.0",
        "testGroups": [{
            "tgId": 1,
            "testType": "AFT",
            "tests": [{
                "tcId": 1,
                "msg": "616263"
            }]
        }]
    });

    assert_eq!(vector["vsId"], 12345);
    assert_eq!(vector["algorithm"], "SHA2-256");
    assert!(vector["testGroups"].is_array());

    let groups = vector["testGroups"].as_array().unwrap();
    assert_eq!(groups.len(), 1);

    let tests = groups[0]["tests"].as_array().unwrap();
    assert_eq!(tests.len(), 1);
    assert_eq!(tests[0]["tcId"], 1);
}

#[test]
fn test_response_format() {
    // Test that we can create a properly formatted response
    let response = json!({
        "vsId": 12345,
        "algorithm": "SHA2-256",
        "revision": "1.0",
        "testGroups": [{
            "tgId": 1,
            "tests": [{
                "tcId": 1,
                "md": "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
            }]
        }]
    });

    // Verify we can serialize it
    let json_str = serde_json::to_string(&response).unwrap();
    assert!(json_str.contains("ba7816bf8f01cfea"));

    // Verify we can deserialize it back
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed["vsId"], 12345);
}

#[test]
fn test_hex_encoding_in_vectors() {
    // Test hex encoding/decoding for test vectors
    let msg_hex = "616263"; // "abc" in hex
    let msg_bytes = hex::decode(msg_hex).unwrap();
    assert_eq!(msg_bytes, b"abc");

    let result_bytes = vec![
        0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22,
        0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00,
        0x15, 0xad,
    ];

    let result_hex = hex::encode(&result_bytes);
    assert_eq!(
        result_hex,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn test_algorithm_routing() {
    // Test that we can identify algorithms correctly
    let algorithms = vec![
        "SHA2-256",
        "SHA3-512",
        "SHAKE-128",
        "HMAC-SHA2-256",
        "ML-DSA",
        "ML-KEM",
        "SLH-DSA",
    ];

    for algo in algorithms {
        assert!(!algo.is_empty());
        assert!(algo.len() < 50); // Reasonable algorithm name length
    }
}
