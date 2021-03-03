extern crate abomonation;

use abomonation::*;

#[test] fn test_array() { _test_pass(vec![[0, 1, 2]; 1024]); }
#[test] fn test_nonzero() { _test_pass(vec![[std::num::NonZeroI32::new(1)]; 1024]); }
#[test] fn test_opt_vec() { _test_pass(vec![Some(vec![0,1,2]), None]); }
#[test] fn test_alignment() { _test_pass(vec![(format!("x"), vec![1,2,3]); 1024]); }
#[test] fn test_alignment_128() { _test_pass(vec![(format!("x"), vec![1u128,2,3]); 1024]); }
#[test] fn test_option_box_u64() { _test_pass(vec![Some(Box::new(0u64))]); }
#[test] fn test_option_vec() { _test_pass(vec![Some(vec![0, 1, 2])]); }
#[test] fn test_u32x4_pass() { _test_pass(vec![((1,2,3),vec![(0u32, 0u32, 0u32, 0u32); 1024])]); }
#[test] fn test_u64_pass() { _test_pass(vec![0u64; 1024]); }
#[test] fn test_u128_pass() { _test_pass(vec![0u128; 1024]); }
#[test] fn test_string_pass() { _test_pass(vec![format!("grawwwwrr!"); 1024]); }
#[test] fn test_vec_u_s_pass() { _test_pass(vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }

#[test] fn test_u64_fail() { _test_fail(vec![0u64; 1024]); }
#[test] fn test_u128_fail() { _test_fail(vec![0u128; 1024]); }
#[test] fn test_string_fail() { _test_fail(vec![format!("grawwwwrr!"); 1024]); }
#[test] fn test_vec_u_s_fail() { _test_fail(vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }

#[test] fn test_array_size() { _test_size(vec![[0, 1, 2]; 1024]); }
#[test] fn test_opt_vec_size() { _test_size(vec![Some(vec![0,1,2]), None]); }
#[test] fn test_alignment_size() { _test_size(vec![(format!("x"), vec![1,2,3]); 1024]); }
#[test] fn test_option_box_u64_size() { _test_size(vec![Some(Box::new(0u64))]); }
#[test] fn test_option_vec_size() { _test_size(vec![Some(vec![0, 1, 2])]); }
#[test] fn test_u32x4_size() { _test_size(vec![((1,2,3),vec![(0u32, 0u32, 0u32, 0u32); 1024])]); }
#[test] fn test_u64_size() { _test_size(vec![0u64; 1024]); }
#[test] fn test_u128_size() { _test_size(vec![0u128; 1024]); }
#[test] fn test_string_size() { _test_size(vec![format!("grawwwwrr!"); 1024]); }
#[test] fn test_vec_u_s_size() { _test_size(vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }

#[test]
fn test_phantom_data_for_non_abomonatable_type() {
    use std::marker::PhantomData;
    struct NotAbomonatable;
    _test_pass(PhantomData::<NotAbomonatable>::default());
}

fn _test_pass<T: Abomonation+Eq>(record: T) {
    let mut bytes = Vec::new();
    unsafe { encode(&record, &mut bytes).unwrap(); }
    {
        let (result, rest) = unsafe { decode::<T>(&mut bytes[..]) }.unwrap();
        assert!(&record == result);
        assert!(rest.len() == 0);
    }
}

fn _test_fail<T: Abomonation>(record: T) {
    let mut bytes = Vec::new();
    unsafe { encode(&record, &mut bytes).unwrap(); }
    bytes.pop();
    assert!(unsafe { decode::<T>(&mut bytes[..]) }.is_none());
}

fn _test_size<T: Abomonation>(record: T) {
    let mut bytes = Vec::new();
    unsafe { encode(&record, &mut bytes).unwrap(); }
    assert_eq!(bytes.len(), measure(&record));
}


#[derive(Eq, PartialEq)]
struct MyStruct {
    a: String,
    b: u64,
    c: Vec<u8>,
}

unsafe_abomonate!(MyStruct : a, b, c);

#[test]
fn test_macro() {
    // create some test data out of abomonation-approved types
    let record = MyStruct{ a: "test".to_owned(), b: 0, c: vec![0, 1, 2] };

    // encode vector into a Vec<u8>
    let mut bytes = Vec::new();
    unsafe { encode(&record, &mut bytes).unwrap(); }

    // decode a &Vec<(u64, String)> from binary data
    if let Some((result, rest)) = unsafe { decode::<MyStruct>(&mut bytes) } {
        assert!(result == &record);
        assert!(rest.len() == 0);
    }
}

use crate::abomonated::Abomonated;
enum MyStructWrap {
    Original(MyStruct),
    Abomonated(Abomonated<MyStruct, Vec<u8>>),
}
use std::ops::Deref;

impl Deref for MyStructWrap {
    type Target = MyStruct;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Original(m) => m,
            Self::Abomonated(a) => &*a,
        }
    }
}

fn de<'a>(bytes: Vec<u8>) -> MyStructWrap {
    MyStructWrap::Abomonated(unsafe { Abomonated::new(bytes) }.unwrap())
}

#[test]
fn test_abominated_enum() {
    // create some test data out of abomonation-approved types
    let record = MyStruct{ a: "test".to_owned(), b: 0, c: vec![0, 1, 2] };

    // encode vector into a Vec<u8>
    let mut bytes = Vec::new();
    unsafe { encode(&record, &mut bytes).unwrap(); }
    let b = bytes.clone();

    let result = de(bytes);
    assert!(*result == record);
}

#[test]
fn test_macro_size() {
    // create some test data out of abomonation-approved types
    let record = MyStruct{ a: "test".to_owned(), b: 0, c: vec![0, 1, 2] };

    // encode vector into a Vec<u8>
    let mut bytes = Vec::new();
    unsafe { encode(&record, &mut bytes).unwrap(); }
    assert_eq!(bytes.len(), measure(&record));
}

#[test]
fn test_multiple_encode_decode() {
    let mut bytes = Vec::new();
    unsafe { encode(&0u32, &mut bytes).unwrap(); }
    unsafe { encode(&7u64, &mut bytes).unwrap(); }
    unsafe { encode(&vec![1,2,3], &mut bytes).unwrap(); }
    unsafe { encode(&"grawwwwrr".to_owned(), &mut bytes).unwrap(); }

    let (t, r) = unsafe { decode::<u32>(&mut bytes) }.unwrap(); assert!(*t == 0);
    let (t, r) = unsafe { decode::<u64>(r) }.unwrap(); assert!(*t == 7);
    let (t, r) = unsafe { decode::<Vec<i32>>(r) }.unwrap(); assert!(*t == vec![1,2,3]);
    let (t, _r) = unsafe { decode::<String>(r) }.unwrap(); assert!(*t == "grawwwwrr".to_owned());
}

#[test]
fn test_net_types() {

    use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};

    let socket_addr4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(128, 0, 0, 1)), 1234);
    let socket_addr6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 1234);

    let mut bytes = Vec::new();

    unsafe { encode(&socket_addr4, &mut bytes).unwrap(); }
    unsafe { encode(&socket_addr6, &mut bytes).unwrap(); }

    let (t, r) = unsafe { decode::<SocketAddr>(&mut bytes) }.unwrap(); assert!(*t == socket_addr4);
    let (t, _r) = unsafe { decode::<SocketAddr>(r) }.unwrap(); assert!(*t == socket_addr6);
}

#[test]
fn test_hash_map() {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    h.insert("aaa".to_string(), 3);
    h.insert("bbb".to_string(), 4);
    let mut bytes = Vec::new();
    unsafe { encode(&h, &mut bytes).unwrap(); }
    println!("{:?}", &bytes);
    let (t, r) = unsafe { decode::<HashMap<String, i32>>(&mut bytes) }.unwrap();
//    assert!(*t == h);
//    println!("{:?}", t);
//    println!("{:?}", &r);

    assert!(r.len() == 0);

    let mut bytes = Vec::new();
    println!("start re encode");
    unsafe { encode(t, &mut bytes).unwrap(); }
    println!("re encode {:?}", bytes);

//    let mut h2 = HashMap::new();
//    h2.insert("aaa".to_string(), "3".to_string());
//    h2.insert("bbb".to_string(), "4".to_string());
//    let mut bytes = Vec::new();
//    unsafe { encode(&h2, &mut bytes).unwrap(); }
//    let (t, r) = unsafe { decode::<HashMap<String, String>>(&mut bytes) }.unwrap(); assert!(*t == h2);
//    println!("{:?}", t);
//    assert!(r.len() == 0);
}

#[test]
fn test_tuple_string_i32() {
//    let x = 3;
//    let x = "aaa".to_string();
    let x = ("aaa".to_string(), 3);
    let mut bytes = Vec::new();
    unsafe { encode(&x, &mut bytes).unwrap(); }
    println!("{:?}", bytes);

}

#[test]
fn test_decode_string() {
    let mut bytes: Vec<u8> = vec![96, 58, 192, 138, 192, 127, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 255, 127, 0, 0, 97, 97, 97];
    let (t, r) = unsafe { decode::<(String, i32)>(&mut bytes) }.unwrap();
    println!("{:?}", t);
}

#[test]
fn test_hm() {
    use std::collections::HashMap;

    let mut bytes: Vec<u8> = vec![111, 139, 146, 221, 149, 26, 92, 124, 165, 93, 15, 24, 242, 156, 41, 211, 3, 0, 0, 0, 0, 0, 0, 0, 128, 2, 80, 246, 251, 127, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 29, 255, 255, 91, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 29, 255, 255, 91, 0, 0, 0, 0, 0, 0, 0, 0, 160, 2, 80, 246, 251, 127, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 98, 98, 98, 3, 0, 0, 0, 0, 0, 0, 0, 240, 1, 80, 246, 251, 127, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 97, 97, 97];
    let (t, r) = unsafe { decode::<HashMap<String, i32>>(&mut bytes) }.unwrap();
    println!("done decode. remains {:?}", r);

    let mut bytes = Vec::new();
    unsafe { encode(t, &mut bytes).unwrap(); }
    println!("re encode {:?}", bytes);

    println!("{:?}", t.values());
}
