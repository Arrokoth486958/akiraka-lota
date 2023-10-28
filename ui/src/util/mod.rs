pub fn as_static_vec<T>(vec: Vec<T>) -> &'static [T] {
    let vec_ptr = vec.as_ptr();
    let vec_len = vec.len();
    let box_vec = Box::new(vec);
    let array: &[T] = unsafe {
        std::slice::from_raw_parts(vec_ptr, vec_len)
    };
    array
}