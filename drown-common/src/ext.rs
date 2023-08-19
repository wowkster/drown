pub trait OptionExt {
    fn none_or<E>(self, e: E) -> Result<(), E>;
}

impl<T> OptionExt for Option<T> {
    fn none_or<E>(self, e: E) -> Result<(), E> {
        match self {
            Some(_) => Err(e),
            None => Ok(()),
        }
    }
}

pub trait StrExt {
    fn matches_or<F, E>(self, validation_fn: F, e: E) -> Result<Self, E>
    where
        F: FnOnce(&Self) -> bool,
        Self: Sized + ToString;

    fn map_empty_to_none(self) -> Option<Self>
    where
        Self: Sized;
}

impl StrExt for &str {
    fn matches_or<F, E>(self, validation_fn: F, e: E) -> Result<Self, E>
    where
        F: FnOnce(&Self) -> bool,
        Self: Sized,
    {
        if validation_fn(&self) {
            Ok(self)
        } else {
            Err(e)
        }
    }

    fn map_empty_to_none(self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl StrExt for String {
    fn matches_or<F, E>(self, validation_fn: F, e: E) -> Result<Self, E>
    where
        F: FnOnce(&Self) -> bool,
        Self: Sized,
    {
        if validation_fn(&self) {
            Ok(self)
        } else {
            Err(e)
        }
    }

    fn map_empty_to_none(self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
