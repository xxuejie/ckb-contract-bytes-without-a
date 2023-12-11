#![no_std]
#![cfg_attr(not(test), no_main)]

use bytes::BytesMut;

use ckb_std::default_alloc;
ckb_std::entry!(program_entry);
default_alloc!();

pub fn program_entry() -> i8 {
    let data = code::load();
    let r = code::run(data.slice(0..16));
    if r == 0 {
        return 0;
    }
    let data2 = code::load();
    let mut buf = BytesMut::new();
    buf.extend(data.slice(8..));
    buf.extend(data2);
    code::run(buf.freeze())
}

// Just some silly code to play with Bytes so it won't be eaten by compilers.
mod code {
    use bytes::Bytes;

    #[inline(never)]
    pub fn load() -> Bytes {
        let mut tx_hash = [0u8; 32];
        let len = ckb_std::syscalls::load_tx_hash(&mut tx_hash, 0).unwrap();
        assert_eq!(len, tx_hash.len());

        Bytes::from(tx_hash.to_vec())
    }

    #[inline(never)]
    pub fn run(data: Bytes) -> i8 {
        let mut i: i8 = 0;
        for d in data {
            i = i.wrapping_add(d as i8);
        }
        i
    }
}
