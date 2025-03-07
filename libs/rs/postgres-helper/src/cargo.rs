pub fn configure() {
    println!("cargo:rerun-if-changed=db");
}
