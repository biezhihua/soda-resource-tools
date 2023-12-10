// 定义一个特质，其中包含一个新的方法
pub(crate) trait ResultExtensions<T, E> {
    fn is_ok_and<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(&T) -> bool;

    fn is_err_and<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(&E) -> bool;

    fn on_err_inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(&E);
}

// 为所有的 `Result<T, E>` 实现这个特质
impl<T, E> ResultExtensions<T, E> for Result<T, E> {
    // 实现方法，检查 Result 是 Ok 且满足给定的条件
    fn is_ok_and<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(&T) -> bool,
    {
        match self {
            Ok(ref value) => predicate(value),
            _ => false,
        }
    }

    // 实现方法，检查 Result 是 Err 且满足给定的条件
    fn is_err_and<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(&E) -> bool,
    {
        match self {
            Err(ref error) => predicate(error),
            _ => false,
        }
    }

    fn on_err_inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(&E),
    {
        match &self {
            Ok(_) => {}
            Err(e) => {
                f(e);
            }
        }
        self
    }
}
