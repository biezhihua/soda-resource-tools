pub(crate) trait OptionExtensions {
    fn is_none_or<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Self::Item) -> bool;

    fn is_some_then<F>(&self, f: F)
    where
        F: FnOnce(&Self::Item);

    fn is_some_mut_then<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self::Item);

    fn is_some_move_then<F>(self, f: F)
    where
        F: FnOnce(Self::Item);

    fn is_none_then<F>(&self, f: F)
    where
        F: FnOnce();

    fn on_none_inspect<F>(self, f: F) -> Self
    where
        F: FnOnce();

    type Item;
}

impl<T> OptionExtensions for Option<T> {
    fn is_none_or<F>(&self, f: F) -> bool
    where
        F: FnOnce(&T) -> bool,
    {
        match self {
            None => true,
            Some(ref x) => f(x),
        }
    }

    /// headers.get("X-RateLimit-Reset").is_some_then(|value| {
    ///     let reset = value.to_str().unwrap().to_string();
    /// });
    fn is_some_then<F>(&self, f: F)
    where
        F: FnOnce(&T),
    {
        match self {
            None => {}
            Some(ref x) => f(x),
        }
    }

    fn is_some_move_then<F>(self, f: F)
    where
        F: FnOnce(Self::Item),
    {
        match self {
            None => {}
            Some(x) => f(x),
        }
    }

    fn is_none_then<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        match self {
            None => {
                f();
            }
            Some(_) => {}
        }
    }

    ///
    /// let api_key = get_api_key().on_none_inspect(|| { tracing::debug!("api key is none"); })?;
    ///
    fn on_none_inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(),
    {
        if self.is_none() {
            f();
        }
        self
    }

    type Item = T;

    fn is_some_mut_then<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self::Item),
    {
        match self {
            None => {}
            Some(ref mut x) => f(x),
        }
    }
}
