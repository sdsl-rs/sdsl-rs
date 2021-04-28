fn main() {
    match sdsl::build() {
        Ok(_) => {}
        Err(e) => panic!("Error: {}", e),
    };
}
