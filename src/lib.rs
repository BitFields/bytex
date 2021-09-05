#![warn(missing_docs)]
#![no_std]


//! Collection of functions for easier 8 bit byte manipulation and representation in no_std environment


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
        array[(9_u8 - position) as usize] = bit::as_char(byte, position as u8);
    }

    array[1] = 'b';
    array[0] = '0';

    array
}

pub mod bit {
    //! Bit manipulations

    /// Panics if position is higher than 7. T
    /// This function is private and for internal use.
    fn check_position(position: u8) {
        if position > 7_u8 {
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
        (target >> position) & 1_u8
    }

    /// Set bit in byte
    ///
    /// # Returns
    ///
    /// dereferenced 'target' param ( *target )
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
    pub fn set(target: &mut u8, position: u8) -> u8 {
        check_position(position);
        *target |= 1_u8 << position;
        *target
    }

    /// Unset bit in byte - returns target
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
    pub fn unset(target: &mut u8, position: u8) -> u8 {
        check_position(position);
        *target &= !(1_u8 << position);
        *target
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
        *target ^= 1_u8 << position;
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
}

pub mod register {
    //! Helper functions for AVR register access
    
    /// Writes directly into register
    /// 
    /// # Examples
    /// 
    /// ``` no_run
    /// use bytex::bit::set;
    /// use bytex::register::{read, write};
    /// 
    /// const DDRD: *mut u8 = 0x2A as *mut u8;
    /// const DDRD0: u8 = 0;
    /// write(DDRD, set(&mut read(DDRD), DDRD0));
    /// ```
    ///
    pub fn write(address: *mut u8, byte: u8) {
        unsafe {
            core::ptr::write_volatile(address, byte);
        }
    }
    
    /// Reads register
    /// 
    /// # Examples
    /// 
    /// ``` no_run
    /// use bytex::register::{read};
    /// 
    /// const DDRD: *mut u8 = 0x2A as *mut u8;
    /// let ddrd = read(DDRD);
    /// ```
    ///
    pub fn read(address: *mut u8) -> u8 {
        unsafe {
            core::ptr::read_volatile(address)
        }
    }
}
