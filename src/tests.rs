use std::net::{Ipv4Addr, Ipv6Addr};
use crate::ip_utils::{add_ip_addresses, subtract_ip_addresses, xor_ipv6_addresses};
use rstest::{rstest, fixture};

#[cfg(test)]
mod tests {
    use super::*;

    struct IpTestCase {
        addr1: [u8; 4],
        addr2: [u8; 4],
        expected: [u8; 4],
    }

    struct Ipv6TestCase {
        addr1: [u8; 16],
        addr2: [u8; 16],
        expected: [u8; 16],
    }

    #[fixture]
    fn local_ip() -> [u8; 4] {
        [192, 168, 1, 1]
    }

    #[fixture]
    fn small_ip() -> [u8; 4] {
        [10, 10, 10, 10]
    }

    #[fixture]
    fn zero_ip() -> [u8; 4] {
        [0, 0, 0, 0]
    }

    #[fixture]
    fn max_ip() -> [u8; 4] {
        [255, 255, 255, 255]
    }

    #[fixture]
    fn one_ip() -> [u8; 4] {
        [1, 1, 1, 1]
    }

    #[fixture]
    fn ipv6_local() -> [u8; 16] {
        [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    }

    #[fixture]
    fn ipv6_other() -> [u8; 16] {
        [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    }

    #[rstest]
    #[case::basic_addition(IpTestCase {
        addr1: local_ip(),
        addr2: small_ip(),
        expected: [202, 178, 11, 11],
    })]
    #[case::overflow_wraps_to_zero(IpTestCase {
        addr1: max_ip(),
        addr2: one_ip(),
        expected: zero_ip(),
    })]
    #[case::add_zero_ip(IpTestCase {
        addr1: local_ip(),
        addr2: zero_ip(),
        expected: local_ip(),
    })]
    fn test_add_ip_addresses(#[case] test_case: IpTestCase) {
        let addr1 = Ipv4Addr::from(test_case.addr1);
        let addr2 = Ipv4Addr::from(test_case.addr2);
        let expected = Ipv4Addr::from(test_case.expected);
        
        let result = add_ip_addresses(addr1, addr2);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::basic_subtraction(IpTestCase {
        addr1: local_ip(),
        addr2: small_ip(),
        expected: [182, 158, 247, 247],
    })]
    #[case::underflow_wraps_around(IpTestCase {
        addr1: zero_ip(),
        addr2: one_ip(),
        expected: max_ip(),
    })]
    #[case::subtract_zero_ip(IpTestCase {
        addr1: local_ip(),
        addr2: zero_ip(),
        expected: local_ip(),
    })]
    #[case::subtract_from_itself(IpTestCase {
        addr1: local_ip(),
        addr2: local_ip(),
        expected: zero_ip(),
    })]
    fn test_subtract_ip_addresses(#[case] test_case: IpTestCase) {
        let addr1 = Ipv4Addr::from(test_case.addr1);
        let addr2 = Ipv4Addr::from(test_case.addr2);
        let expected = Ipv4Addr::from(test_case.expected);
        
        let result = subtract_ip_addresses(addr1, addr2);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::basic_xor(Ipv6TestCase {
        addr1: ipv6_local(),
        addr2: ipv6_other(),
        expected: [
            0xde, 0x81, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
    })]
    #[case::xor_with_same(Ipv6TestCase {
        addr1: ipv6_local(),
        addr2: ipv6_local(),
        expected: [0; 16],  // XOR with same number always results in 0
    })]
    fn test_xor_ipv6_addresses(#[case] test_case: Ipv6TestCase) {
        let addr1 = Ipv6Addr::from(test_case.addr1);
        let addr2 = Ipv6Addr::from(test_case.addr2);
        let expected = Ipv6Addr::from(test_case.expected);
        
        let result = xor_ipv6_addresses(addr1, addr2);
        assert_eq!(result, expected);
    }
}
