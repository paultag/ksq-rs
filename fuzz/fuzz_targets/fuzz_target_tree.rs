#![no_main]
#![feature(iter_array_chunks)]

use ksq::Tree;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let data: Vec<u16> = {
        data.iter()
            .cloned()
            .array_chunks()
            .map(|v| u16::from_be_bytes(v).try_into().unwrap())
            .collect()
    };

    let _ = Tree::from(&data);
});
