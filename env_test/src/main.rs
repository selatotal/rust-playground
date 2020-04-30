fn main() {
	let version = option_env!("TEST").unwrap_or("1");
    	println!("Hello, {}!", version);
}
