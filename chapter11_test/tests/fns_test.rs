
// in seperated crate, so to import the crate to be tested
use chptrtest;

// #[cfg(test)]
// mod tests {

    fn local_add_1(input : i32) -> i32 {
        input + 1
    }

    #[test]
    fn it_works() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_local_add_1() {
        assert_eq!(2 + 1, local_add_1(2));
    }

    #[test]
    fn test_add_2() {
        assert_eq!(3 + 2, chptrtest::adder::add_2(3));
    }

// }

