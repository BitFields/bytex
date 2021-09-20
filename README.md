# bytex

## bytex::repr

``` rust
let x: u8 = 0b1000_0010;
assert_eq!(['0','b','1','0','0','0','0','0','1','0'], bytex::repr(x));
```

## bytex::bit

### bytex::bit::get

``` rust
let x: u8 = 0b0000_0001;
let y: u8 = bytex::bit::get(x, 0);
assert_eq!(y, 1);
```

### bytex::bit::set

``` rust
let mut x: u8 = 0b0000_0000;
bytex::bit::set(&mut x, 0);
assert_eq!(x, 0b0000_0001);
```

### bytex::bit::unset

``` rust
let mut x: u8 = 0b0000_0001;
bytex::bit::unset(&mut x, 0);
assert_eq!(x, 0b0000_0000);
```

### bytex::bit::toggle

``` rust
let mut x: u8 = 0b0000_0001;
bytex::bit::toggle(&mut x, 0);
assert_eq!(x, 0b0000_0000);
```

### bytex::bit::as_char

``` rust
let x: u8 = 0b0000_0001;
let y: char = bytex::bit::as_char(x, 0);
assert_eq!(y, '1');
```

## bytex::register

### bytex::register::write

``` rust
use bytex::bit::set;
use bytex::register::{read, write};

const DDRD: *mut u8 = 0x2A as *mut u8;
const DDRD0: u8 = 0;
write(DDRD, set(&mut read(DDRD), DDRD0));
```

### bytex::register::read

``` rust
use bytex::register::{read};

const DDRD: *mut u8 = 0x2A as *mut u8;
let ddrd = read(DDRD);
```
