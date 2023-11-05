pub fn octets_to_string(octets: [u8; 4]) -> String {
    format!(
        "{}.{}.{}.{}",
        octets[0].to_string(),
        octets[1].to_string(),
        octets[2].to_string(),
        octets[3].to_string()
    )
}
