#![no_std]
#![no_main]

use bytex::register;
use bytex::bit;

// DDRD – The Port D Data Direction Register
pub const DDRD: u8 = 0x0A;
pub const DDD7: u8 = 7;
// PORTD – The Port D Data Register
pub const PORTD: u8 = 0x0B;
pub const PORTD7: u8 = 7;


fn setup() {
    // set DDD7 to 1 to enable pin as output
    register::write(DDRD, bit::set(&mut bit::read(DDRD), 7));
}


#[no_mangle]
pub extern fn main() {

    // DDRD = DDRD | 1 << DDD7;

    setup();

    let (mut counter0, mut counter1, mut counter2) = 0u8;

    loop {
        counter0 += 1;

        if counter0 == 255 {
            counter0 = 0;
            counter1 += 1;
        }
        if counter1 == 255 {
            counter1 = 0;
            counter2 += 1;
        }
        if counter2 == 255 {
            register::write(PORTD, bit::toggle(&mut register::read(PORTD), PORTD7));
            counter2 = 0;
        }
    }
}
