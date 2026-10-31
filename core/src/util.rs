pub fn typename<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}