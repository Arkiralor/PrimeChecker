//! General purpose utility functions.


pub fn unique_elements_vector<
    T: std::ops::Add<Output = T> + std::fmt::Debug + std::cmp::PartialEq,
>(
    _list: Vec<T>,
) -> Vec<T> {
    //! Find all the UNIQUE elements in a given vector of Datatype T*
    //      * where T has the following attributes:
    //          Add()
    //          Debug()
    //          PartialEq()

    //  Arguments:
    //      _list: Vec<T>

    //  Returns:
    //      Vec<T>n
    let mut unique_list: Vec<T> = Vec::new();
    for item in _list {
        if !unique_list.contains(&item) {
            unique_list.push(item);
        }
    }
    return unique_list;
}