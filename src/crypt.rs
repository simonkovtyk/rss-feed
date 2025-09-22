pub fn get_hash<S> (data: &S) -> String
where S: bincode::enc::Encode {
  let bytes = bincode::encode_to_vec(data, bincode::config::standard()).unwrap();

  blake3::Hasher::new()
    .update(&bytes)
    .finalize()
    .to_string()
}