#[cfg(test)]
macro_rules! assert_panic {
    ($f:expr) => {
        assert!(std::panic::catch_unwind(|| $f).is_err());
    };
}

#[cfg(test)]
pub(crate) use assert_panic;
