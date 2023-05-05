use crate::libs::utils;

#[test]
pub fn test_unique_elements_vector(){
    //! Tests the unique_elements_vector function with an array of u64.
    let unique_arr: Vec<u64> = vec![1,2,3,4,5,6,7,8,9,10];
    let non_unique_arr: Vec<u64> = vec![1,2,3,4,5,6,7,8,9,10,1,2,3,4];

    let unique = utils::unique_elements_vector(non_unique_arr);
    assert_eq!(unique, unique_arr)
}

#[test]
pub fn test_unique_elements_vector_2(){
    //! Tests the unique_elements_vector function with an array of char.
    let unique_arr: Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j'];
    let non_unique_arr: Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j','a','b','c','d'];

    let unique = utils::unique_elements_vector(non_unique_arr);
    assert_eq!(unique, unique_arr)
}