pub mod bit {

	/// Panics if position is higher than 7
	fn check_position(position: u8) {
		if position > 7 {
			panic!("position cannot be higher than 7");
		}
	}

	/// Get bit from byte
	///
	/// # Panics
	/// 
	/// Panics when position is higher than 7
	/// 
	/// # Examples
	/// ```
	/// let x: u8 = 0b0000_0001;
	/// let y: u8 = bit::get(x, 0);
	/// assert_eq!(y, 1);
	/// ```
	pub fn get(target: u8, position: u8) -> u8 {
		check_position(position);
		(target >> position) & 1
	}

	/// Set bit in byte
	///
	/// # Panics
	/// 
	/// Panics when position is higher than 7
	/// 
	/// # Examples
	/// ```
	/// let x: u8 = 0b0000_0000;
	/// bit::set(x, 0);
	/// assert_eq!(x, 0b0000_0001);
	/// ```
	pub fn set(target: &mut u8, position: u8) {
		check_position(position);
		*target |= 1 << position;
	}

	/// Unset bit in byte
	///
	/// # Panics
	/// 
	/// Panics when position is higher than 7
	/// 
	/// # Examples
	/// ```
	/// let x: u8 = 0b0000_0001;
	/// bit::unset(x, 0);
	/// assert_eq!(x, 0b0000_0000);
	/// ```
	pub fn unset(target: &mut u8, position: u8) {
		check_position(position);
		*target &= !(1 << position);
	}

	/// Toggle bit in byte
	///
	/// # Panics
	/// 
	/// Panics when position is higher than 7
	/// 
	/// # Examples
	/// ```
	/// let x: u8 = 0b0000_0001;
	/// bit::toggle(x, 0);
	/// assert_eq!(x, 0b0000_0000);
	/// ```
	pub fn toggle(target: &mut u8, position: u8) {
		check_position(position);
		*target ^= 1 << position;
	}

	/// Returns bit represented as char e.g '0' or '1'
	///
	/// # Panics
	/// 
	/// Panics when position is higher than 7 or at unexpected bit values (not 0 or 1)
	/// 
	/// # Examples
	/// ```
	/// let x: u8 = 0b0000_0001;
	/// let y: char = bit::as_char(x, 0);
	/// assert_eq!(y, '1');
	/// ```
	pub fn as_char(target: u8, position: u8) -> char {
		check_position(position);

		match get(target, position) {
			0 => '0',
			1 => '1',
			_ => panic!("Expected 0 or 1"),
		}
	}

	#[test]
	fn test_set() {
		let mut x: u8 = 0b0000_0000;

		set(&mut x, 0);
		assert_eq!(x, 0b0000_0001);

		set(&mut x, 7);
		assert_eq!(x, 0b1000_0001);
	}

	#[test]
	#[should_panic]
	fn test_set_fail() {
		let mut x: u8 = 0b0000_0000;

		set(&mut x, 10);
	}

	#[test]
	fn test_unset() {
		let mut x: u8 = 0b1111_1111;

		unset(&mut x, 0);
		assert_eq!(x, 0b1111_1110);

		unset(&mut x, 7);
		assert_eq!(x, 0b0111_1110);
	}

	#[test]
	#[should_panic]
	fn test_unset_fail() {
		let mut x: u8 = 0b0000_0001;

		unset(&mut x, 10);
	}

	#[test]
	fn test_toggle() {
		let mut x: u8 = 0b1000_0001;
		
		toggle(&mut x, 0);
		assert_eq!(x, 0b1000_0000);

		toggle(&mut x, 7);
		assert_eq!(x, 0b0000_0000);
	}

	#[test]
	#[should_panic]
	fn test_toggle_fail() {
		let mut x: u8 = 0b0000_0001;
		toggle(&mut x, 10);
	}

	#[test]
	fn test_as_char() {
		let x: u8 = 0b1000_0001;
		let mut y: char = as_char(x, 0);

		assert_eq!(y, '1');
		
		y = as_char(x, 1);
		assert_eq!(y, '0');

		y = as_char(x, 7);
		assert_eq!(y, '1');
	}

	#[test]
	#[should_panic]
	fn test_as_char_fail() {
		let x: u8 = 0b0000_0001;
		as_char(x, 10);
	}
}

/// Represents byte as String in the format '0bxxxxxxxx'
///
/// # Examples
///
/// ```rust
//// let x: u8 = 0b0000_0001;
//// assert_eq("0b00000001", repr(x));
/// ```
pub fn repr(byte: u8) -> String {
	let mut array: [char; 10] = ['\0'; 10];

	for position in 0..=7 {
		array[position] = bit::as_char(byte, position as u8);
	}

	array[8] = 'b';
	array[9] = '0';

	array.iter().rev().collect()
}

#[test]
fn test_repr() {
	let x: u8 = 0b1000_0000;

	assert_eq!("0b10000000", repr(x));
}