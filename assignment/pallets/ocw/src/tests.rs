use sp_runtime::Permill;


#[test]
pub fn permill_test() {
	let s = "30.8287640368510822";
	let price_usd: Vec<&str> = s.split(".").collect();
	let price_usd_num: u64 = price_usd[0].parse().unwrap();
	let price_usd_permill: Permill = Permill::from_parts(price_usd[1][..6].parse::<u32>().unwrap());
	assert_eq!(price_usd_num, 30);
	assert_eq!(price_usd_permill, Permill::from_parts(828764))
}
#[test]
pub fn permill2_test() {
	let s = "28.96806065";
	let price_usd: Vec<&str> = s.split(".").collect();
	let price_usd_num: u64 = price_usd[0].parse().unwrap();
	let price_usd_permill: Permill = Permill::from_parts(price_usd[1][..6].parse::<u32>().unwrap());
	assert_eq!(price_usd_num, 28);
	assert_eq!(price_usd_permill, Permill::from_parts(968060))
}
