mod algo;

fn main() {
    let msg = String::from("hello World!").as_bytes().to_vec();
    let key = algo::generate_key(msg.len());
    let encrypted_msg = algo::encrypt(msg.clone(), key.clone());
    let decrypt_test = algo::decrypt(encrypted_msg, key);
    println!("{}", String::from_utf8_lossy(&decrypt_test));
    println!("It works!");
}
