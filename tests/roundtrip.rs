// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use memcomparable::{from_slice, to_vec};

#[test]
fn test_bytes_roundtrip_empty() {
    let original: Vec<u8> = vec![];
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_single_byte() {
    let original: Vec<u8> = vec![0x42];
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_small() {
    let original: Vec<u8> = vec![0x01, 0x02, 0x03];
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_exactly_one_chunk() {
    // Exactly 8 bytes (one chunk)
    let original: Vec<u8> = (0..8).collect();
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_two_chunks() {
    // 10 bytes (two chunks)
    let original: Vec<u8> = (0..10).collect();
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_multiple_chunks() {
    // 64 bytes (8 chunks)
    let original: Vec<u8> = (0..64).collect();
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_large() {
    // 100 bytes
    let original: Vec<u8> = (0..100).map(|i| (i % 256) as u8).collect();
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_very_large() {
    // 1000 bytes
    let original: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    let serialized = to_vec(&original).unwrap();
    let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_roundtrip_various_lengths() {
    let lengths = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 16, 32, 56, 57, 64, 65, 100, 200, 500, 1000,
    ];

    for len in lengths {
        let original: Vec<u8> = (0..len).map(|i| (i % 256) as u8).collect();
        let serialized = to_vec(&original).unwrap();
        let deserialized: Vec<u8> = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original, "Failed for length: {}", len);
    }
}

#[test]
fn test_i8_roundtrip() {
    let test_cases = vec![i8::MIN, -128, -64, -1, 0, 1, 64, 127, i8::MAX];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: i8 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_u8_roundtrip() {
    let test_cases = vec![0u8, 1, 64, 127, 128, 255, u8::MAX];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: u8 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_i16_roundtrip() {
    let test_cases = vec![i16::MIN, -32768, -16384, -1, 0, 1, 16384, 32767, i16::MAX];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: i16 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_u16_roundtrip() {
    let test_cases = vec![0u16, 1, 16384, 32767, 32768, 65535, u16::MAX];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: u16 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_i32_roundtrip() {
    let test_cases = vec![
        i32::MIN,
        -2147483648,
        -1073741824,
        -1,
        0,
        1,
        1073741824,
        2147483647,
        i32::MAX,
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: i32 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_u32_roundtrip() {
    let test_cases = vec![
        0u32,
        1,
        1073741824,
        2147483647,
        2147483648,
        4294967295,
        u32::MAX,
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: u32 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_i64_roundtrip() {
    let test_cases = vec![
        i64::MIN,
        -9223372036854775808,
        -4611686018427387904,
        -1,
        0,
        1,
        4611686018427387904,
        9223372036854775807,
        i64::MAX,
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: i64 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_u64_roundtrip() {
    let test_cases = vec![
        0u64,
        1,
        4611686018427387904,
        9223372036854775807,
        9223372036854775808,
        18446744073709551615,
        u64::MAX,
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: u64 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_i128_roundtrip() {
    let test_cases = vec![
        i128::MIN,
        -170141183460469231731687303715884105728,
        -1,
        0,
        1,
        170141183460469231731687303715884105727,
        i128::MAX,
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: i128 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_u128_roundtrip() {
    let test_cases = vec![
        0u128,
        1,
        170141183460469231731687303715884105727,
        170141183460469231731687303715884105728,
        340282366920938463463374607431768211455,
        u128::MAX,
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: u128 = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_mixed_types_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MixedTypes {
        bytes: Vec<u8>,
        i8_val: i8,
        u8_val: u8,
        i16_val: i16,
        u16_val: u16,
        i32_val: i32,
        u32_val: u32,
        i64_val: i64,
        u64_val: u64,
        i128_val: i128,
        u128_val: u128,
    }

    let original = MixedTypes {
        bytes: vec![0x01, 0x02, 0x03, 0x04, 0x05],
        i8_val: -128,
        u8_val: 255,
        i16_val: -32768,
        u16_val: 65535,
        i32_val: -2147483648,
        u32_val: 4294967295,
        i64_val: -9223372036854775808,
        u64_val: 18446744073709551615,
        i128_val: i128::MIN,
        u128_val: u128::MAX,
    };

    let serialized = to_vec(&original).unwrap();
    let deserialized: MixedTypes = from_slice(&serialized).unwrap();
    assert_eq!(deserialized, original);
}

#[test]
fn test_bytes_with_other_types_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct BytesWithTypes {
        data: Vec<u8>,
        count: u32,
        id: i64,
    }

    let test_cases = vec![
        BytesWithTypes {
            data: vec![],
            count: 0,
            id: 0,
        },
        BytesWithTypes {
            data: vec![0x42],
            count: 1,
            id: 1,
        },
        BytesWithTypes {
            data: (0..100).collect(),
            count: 100,
            id: -100,
        },
        BytesWithTypes {
            data: (0..1000).map(|i| (i % 256) as u8).collect(),
            count: 1000,
            id: 123456789,
        },
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: BytesWithTypes = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_string_u32_u8_i32_u16_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct StringWithNumbers {
        name: String,
        count: u32,
        flags: u8,
        offset: i32,
        port: u16,
    }

    let test_cases = vec![
        StringWithNumbers {
            name: "".to_string(),
            count: 0,
            flags: 0,
            offset: 0,
            port: 0,
        },
        StringWithNumbers {
            name: "hello".to_string(),
            count: 42,
            flags: 255,
            offset: -100,
            port: 8080,
        },
        StringWithNumbers {
            name: "world".to_string(),
            count: u32::MAX,
            flags: u8::MAX,
            offset: i32::MIN,
            port: u16::MAX,
        },
        StringWithNumbers {
            name: "test string with spaces".to_string(),
            count: 123456,
            flags: 128,
            offset: -2147483648,
            port: 443,
        },
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: StringWithNumbers = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_tuple_string_u32_u8_i32_u16_roundtrip() {
    let test_cases = vec![
        ("".to_string(), 0u32, 0u8, 0i32, 0u16),
        ("hello".to_string(), 42u32, 255u8, -100i32, 8080u16),
        ("world".to_string(), u32::MAX, u8::MAX, i32::MIN, u16::MAX),
        ("test".to_string(), 123456u32, 128u8, -2147483648i32, 443u16),
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: (String, u32, u8, i32, u16) = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_multiple_strings_with_numbers_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MultipleStrings {
        first: String,
        second: String,
        count: u32,
        id: u8,
        value: i32,
        port: u16,
    }

    let test_cases = vec![
        MultipleStrings {
            first: "".to_string(),
            second: "".to_string(),
            count: 0,
            id: 0,
            value: 0,
            port: 0,
        },
        MultipleStrings {
            first: "hello".to_string(),
            second: "world".to_string(),
            count: 42,
            id: 255,
            value: -100,
            port: 8080,
        },
        MultipleStrings {
            first: "first string".to_string(),
            second: "second string".to_string(),
            count: u32::MAX,
            id: u8::MAX,
            value: i32::MAX,
            port: u16::MAX,
        },
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: MultipleStrings = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_complex_mixed_types_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ComplexMixed {
        name: String,
        bytes: Vec<u8>,
        count: u32,
        flags: u8,
        offset: i32,
        port: u16,
        id: i64,
        timestamp: u64,
    }

    let test_cases = vec![
        ComplexMixed {
            name: "".to_string(),
            bytes: vec![],
            count: 0,
            flags: 0,
            offset: 0,
            port: 0,
            id: 0,
            timestamp: 0,
        },
        ComplexMixed {
            name: "test".to_string(),
            bytes: vec![0x01, 0x02, 0x03],
            count: 42,
            flags: 255,
            offset: -100,
            port: 8080,
            id: -123456789,
            timestamp: 1234567890,
        },
        ComplexMixed {
            name: "complex test case".to_string(),
            bytes: (0..100).collect(),
            count: u32::MAX,
            flags: u8::MAX,
            offset: i32::MIN,
            port: u16::MAX,
            id: i64::MAX,
            timestamp: u64::MAX,
        },
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: ComplexMixed = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_all_numeric_types_together_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct AllNumericTypes {
        i8_val: i8,
        u8_val: u8,
        i16_val: i16,
        u16_val: u16,
        i32_val: i32,
        u32_val: u32,
        i64_val: i64,
        u64_val: u64,
        i128_val: i128,
        u128_val: u128,
    }

    let test_cases = vec![
        AllNumericTypes {
            i8_val: 0,
            u8_val: 0,
            i16_val: 0,
            u16_val: 0,
            i32_val: 0,
            u32_val: 0,
            i64_val: 0,
            u64_val: 0,
            i128_val: 0,
            u128_val: 0,
        },
        AllNumericTypes {
            i8_val: -128,
            u8_val: 255,
            i16_val: -32768,
            u16_val: 65535,
            i32_val: -2147483648,
            u32_val: 4294967295,
            i64_val: -9223372036854775808,
            u64_val: 18446744073709551615,
            i128_val: i128::MIN,
            u128_val: u128::MAX,
        },
        AllNumericTypes {
            i8_val: i8::MAX,
            u8_val: u8::MAX,
            i16_val: i16::MAX,
            u16_val: u16::MAX,
            i32_val: i32::MAX,
            u32_val: u32::MAX,
            i64_val: i64::MAX,
            u64_val: u64::MAX,
            i128_val: i128::MAX,
            u128_val: u128::MAX,
        },
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: AllNumericTypes = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_strings_and_all_numeric_types_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct StringsAndNumbers {
        first: String,
        second: String,
        third: String,
        i8_val: i8,
        u8_val: u8,
        i16_val: i16,
        u16_val: u16,
        i32_val: i32,
        u32_val: u32,
        i64_val: i64,
        u64_val: u64,
    }

    let test_cases = vec![
        StringsAndNumbers {
            first: "".to_string(),
            second: "".to_string(),
            third: "".to_string(),
            i8_val: 0,
            u8_val: 0,
            i16_val: 0,
            u16_val: 0,
            i32_val: 0,
            u32_val: 0,
            i64_val: 0,
            u64_val: 0,
        },
        StringsAndNumbers {
            first: "hello".to_string(),
            second: "world".to_string(),
            third: "test".to_string(),
            i8_val: -128,
            u8_val: 255,
            i16_val: -32768,
            u16_val: 65535,
            i32_val: -2147483648,
            u32_val: 4294967295,
            i64_val: -9223372036854775808,
            u64_val: 18446744073709551615,
        },
        StringsAndNumbers {
            first: "first string".to_string(),
            second: "second string".to_string(),
            third: "third string".to_string(),
            i8_val: i8::MAX,
            u8_val: u8::MAX,
            i16_val: i16::MAX,
            u16_val: u16::MAX,
            i32_val: i32::MAX,
            u32_val: u32::MAX,
            i64_val: i64::MAX,
            u64_val: u64::MAX,
        },
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: StringsAndNumbers = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_tuple_mixed_types_roundtrip() {
    let test_cases = vec![
        ("".to_string(), 0u32, 0u8, 0i32, 0u16, vec![] as Vec<u8>),
        (
            "hello".to_string(),
            42u32,
            255u8,
            -100i32,
            8080u16,
            vec![0x01, 0x02, 0x03],
        ),
        (
            "world".to_string(),
            u32::MAX,
            u8::MAX,
            i32::MIN,
            u16::MAX,
            (0..100).collect(),
        ),
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: (String, u32, u8, i32, u16, Vec<u8>) = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}

#[test]
fn test_nested_mixed_types_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Inner {
        value: i32,
        count: u16,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Outer {
        name: String,
        inner: Inner,
        bytes: Vec<u8>,
        id: u32,
        flag: u8,
    }

    let test_cases = vec![
        Outer {
            name: "".to_string(),
            inner: Inner { value: 0, count: 0 },
            bytes: vec![],
            id: 0,
            flag: 0,
        },
        Outer {
            name: "test".to_string(),
            inner: Inner {
                value: -100,
                count: 8080,
            },
            bytes: vec![0x01, 0x02, 0x03],
            id: 42,
            flag: 255,
        },
        Outer {
            name: "complex nested".to_string(),
            inner: Inner {
                value: i32::MAX,
                count: u16::MAX,
            },
            bytes: (0..64).collect(),
            id: u32::MAX,
            flag: u8::MAX,
        },
    ];

    for original in test_cases {
        let serialized = to_vec(&original).unwrap();
        let deserialized: Outer = from_slice(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }
}
