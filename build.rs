fn main()
{
    let path = std::env::var("CARGO_BIN_FILE_TEST").unwrap();
    //make the artifacts available to the test at runtime
    println!("cargo:rustc-env={}={}", "CRATE_BIN", path);
}
