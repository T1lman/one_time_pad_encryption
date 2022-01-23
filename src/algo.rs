pub fn encrypt(msg: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let len = msg.len();
    assert_eq!(len, key.len());
    let mut returnvec: Vec<u8> = Vec::new();
    for i in 0..len {
        let num = ((msg[i] as i16 + key[i] as i16) % 255) as u8;
        returnvec.push(num as u8);
    }
    returnvec
}

pub fn decrypt(msg: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut returnvec: Vec<u8> = Vec::new();
    for i in 0..msg.len() {
        if msg[i] < key[i] {
            let num = ((msg[i] as i8 - key[i] as i8) as u8) - 1;
            returnvec.push(num)
        } else {
            let num = msg[i] - key[i];
            returnvec.push(num)
        }
    }
    returnvec
}

pub fn generate_key(len: usize) -> Vec<u8> {
    use rand::Rng;
    let mut returnvec: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        returnvec.push(rng.gen())
    }

    returnvec
}
