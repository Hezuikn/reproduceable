fn main()
{
    let path = std::env::var("CARGO_BIN_FILE_TEST").unwrap();
    println!("cargo:rustc-env={}={}", "CRATE_BIN", path);
}
