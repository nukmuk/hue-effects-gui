pub fn get_psk(hint: &[u8]) -> Result<Vec<u8>, webrtc::dtls::Error> {
    let contents =
        std::fs::read_to_string("psk.txt").expect("Should have been able to read the file");

    println!("read file: {contents}");

    Ok(hex::decode(contents).unwrap())
}
