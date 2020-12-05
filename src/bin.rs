fn main() {
    let unparsed_file = std::fs::read_to_string("test.nsconf").unwrap();
    let properties = nsconfig::parse(&unparsed_file);
    println!("{:#?}", properties);
}
