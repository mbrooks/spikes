use std::hash::{Hash, Hasher};
use fasthash::{xx, XXHasher};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s: XXHasher = Default::default();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let h = xx::hash64(b"hello world\xff");
    println!("Value: {}", h);
    assert_eq!(h, hash(&"hello world"));

    let buf: Vec<u8> = "SIP/2.0 200 OK \
    Via: SIP/2.0/UDP 127.0.0.1:5061;branch=z9hG4bK-7764-49994-0 \
    From: sipp <sip:sipp@127.0.0.1:5061>;tag=49994 \
    To: <sip:30@127.0.0.1> \
    Call-ID: 49994-7764@127.0.0.1 \
    CSeq: 1 OPTIONS \
    Content-Length: 0 \
    Server: rsip".into();

    let h = xx::hash64(buf);
    println!("Value: {}", h);
}
