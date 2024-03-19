use learn2code::gcd;

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11, 3 * 5 * 11 * 13), 3 * 5 * 11);
}
