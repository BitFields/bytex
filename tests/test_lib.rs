#[cfg(test)] // The module is only compiled when testing.

mod test {
    use bytex;

    #[test]
    fn test_repr() {
        let x: u8 = 0b1000_0010;

        assert_eq!(['0', 'b', '1', '0', '0', '0', '0', '0', '1', '0'], bytex::repr(x));
    }

    #[test]
    fn test_set() {
        let mut x: u8 = 0b0000_0000;

        bytex::bit::set(&mut x, 0);
        assert_eq!(x, 0b0000_0001);

        bytex::bit::set(&mut x, 7);
        assert_eq!(x, 0b1000_0001);
    }

    #[test]
    #[should_panic]
    fn test_set_fail() {
        let mut x: u8 = 0b0000_0000;

        bytex::bit::set(&mut x, 10);
    }

    #[test]
    fn test_unset() {
        let mut x: u8 = 0b1111_1111;

        bytex::bit::unset(&mut x, 0);
        assert_eq!(x, 0b1111_1110);

        bytex::bit::unset(&mut x, 7);
        assert_eq!(x, 0b0111_1110);
    }

    #[test]
    #[should_panic]
    fn test_unset_fail() {
        let mut x: u8 = 0b0000_0001;

        bytex::bit::unset(&mut x, 10);
    }

    #[test]
    fn test_toggle() {
        let mut x: u8 = 0b1000_0001;

        bytex::bit::toggle(&mut x, 0);
        assert_eq!(x, 0b1000_0000);

        bytex::bit::toggle(&mut x, 7);
        assert_eq!(x, 0b0000_0000);
    }

    #[test]
    #[should_panic]
    fn test_toggle_fail() {
        let mut x: u8 = 0b0000_0001;
        bytex::bit::toggle(&mut x, 10);
    }

    #[test]
    fn test_as_char() {
        let x: u8 = 0b1000_0001;
        let mut y: char = bytex::bit::as_char(x, 0);

        assert_eq!(y, '1');

        y = bytex::bit::as_char(x, 1);
        assert_eq!(y, '0');

        y = bytex::bit::as_char(x, 7);
        assert_eq!(y, '1');
    }

    #[test]
    #[should_panic]
    fn test_as_char_fail() {
        let x: u8 = 0b0000_0001;
        bytex::bit::as_char(x, 10);
    }
}
