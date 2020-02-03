#[macro_export]

macro_rules! base_test {
    ($sorter:ident) => {
        #[test]
        pub fn test1() {
            let mut test_case_1 = vec![2, 3, 5, 4, 1];
            let result_1 = vec![1, 2, 3, 4, 5];

            $sorter(&mut test_case_1);
            assert_eq!(test_case_1, result_1);

        }

        #[test]
        pub fn test2() {
            let mut test_case_2 = vec![1, 2, 3];
            let result_2 = vec![1, 2, 3];

            $sorter(&mut test_case_2);
            assert_eq!(test_case_2, result_2);
        }

        #[test]
        pub fn test3() {
            let mut test_case_3 = vec![3, 2];
            let result_3 = vec![2, 3];

            $sorter(&mut test_case_3);
            assert_eq!(test_case_3, result_3);
        }

        #[test]
        pub fn test4() {
            let mut test_case_4 = vec![2, 3, 1];
            let result_4 = vec![1, 2, 3];

            $sorter(&mut test_case_4);
            assert_eq!(test_case_4, result_4);
        }

        #[test]
        pub fn test5() {
            let mut test_case_4 = vec![2, 3, 1, 6, 10, 4, 11, 7, 15];
            let result_4 = vec![1, 2, 3, 4, 6, 7, 10, 11, 15];

            $sorter(&mut test_case_4);
            assert_eq!(test_case_4, result_4);
        }
    };
}
