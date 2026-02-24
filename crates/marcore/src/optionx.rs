pub trait OptionExt<T> {
    fn or_try<E, F>(self, f: F) -> Result<T,E>
    where 
        F: FnOnce() -> Result<T, E>;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn or_try<E, F>(self, f: F) -> Result<T,E>
        where 
            F: FnOnce() -> Result<T, E> {
        match self {
            Some(v) => Ok(v),
            None => f()
        }
    }
}