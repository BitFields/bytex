#![no_std]
pub mod bit {
	//! Bit manipulations

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
    ///
	/// ```
	/// let x: u8 = 0b0000_0001;
	/// let y: u8 = bytex::bit::get(x, 0);
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
    ///
	/// ```
	/// let mut x: u8 = 0b0000_0000;
	/// bytex::bit::set(&mut x, 0);
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
    ///
	/// ```
	/// let mut x: u8 = 0b0000_0001;
	/// bytex::bit::unset(&mut x, 0);
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
    ///
	/// ```
	/// let mut x: u8 = 0b0000_0001;
	/// bytex::bit::toggle(&mut x, 0);
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
    ///
	/// ```
	/// let x: u8 = 0b0000_0001;
	/// let y: char = bytex::bit::as_char(x, 0);
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

/// Represents byte as [char; 10] in the format '0bxxxxxxxx' suitable for printing
///
/// # Examples
///
/// ```
/// let x: u8 = 0b1000_0010;
/// assert_eq!(['0','b','1','0','0','0','0','0','1','0'], bytex::repr(x));
/// ```
pub fn repr(byte: u8) -> [char; 10] {
	let mut array: [char; 10] = ['\0'; 10];

	for position in 0..=7 {
		array[7 - position + 2] = bit::as_char(byte, position as u8);
	}

	array[1] = 'b';
	array[0] = '0';

	array
}

#[test]
fn test_repr() {
	let x: u8 = 0b1000_0010;

	assert_eq!(['0','b','1','0','0','0','0','0','1','0'], repr(x));
}