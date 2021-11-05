fn main()
{
	unimplemented!();
}

#[test]
fn test()
{
    let is_success = std::process::Command::new(std::env::var("CRATE_BIN").unwrap())
		.status().unwrap().success();
    assert!(is_success);
}
