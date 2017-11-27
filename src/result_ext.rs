use core::fmt::Display;

use {Fail, Compat, Context};

/// Extension methods for Result.
pub trait ResultExt<T, E> {
    /// Wraps the error in `Compat` to make it compatible with older error
    /// handling APIs that expect std::error::Error;
    fn compat(self) -> Result<T, Compat<E>>;

    /// Wraps the error type in a context type.
    fn context<D>(self, context: D) -> Result<T, Context<D>> where
        D: Display + Send + Sync + 'static;

    /// Wraps the error type in a context type generated by looking at the
    /// error value.
    fn with_context<F, D>(self, f: F) -> Result<T, Context<D>> where
        F: FnOnce(&E) -> D,
        D: Display + Send + Sync + 'static;
}

impl<T, E> ResultExt<T, E> for Result<T, E> where
    E: Fail,
{
    fn compat(self) -> Result<T, Compat<E>> {
        self.map_err(|err| err.compat())
    }

    fn context<D>(self, context: D) -> Result<T, Context<D>> where
        D: Display + Send + Sync + 'static
    {
        self.map_err(|failure| failure.context(context))
    }

    fn with_context<F, D>(self, f: F) -> Result<T, Context<D>> where
        F: FnOnce(&E) -> D,
        D: Display + Send + Sync + 'static
    {
        self.map_err(|failure| {
            let context = f(&failure);
            failure.context(context)
        })
    }
}

with_std! {
    use Error;

    impl<T> ResultExt<T, Error> for Result<T, Error> {
        fn compat(self) -> Result<T, Compat<Error>> {
            self.map_err(|err| err.compat())
        }

        fn context<D>(self, context: D) -> Result<T, Context<D>> where
            D: Display + Send + Sync + 'static
        {
            self.map_err(|failure| failure.context(context))
        }

        fn with_context<F, D>(self, f: F) -> Result<T, Context<D>> where
            F: FnOnce(&Error) -> D,
            D: Display + Send + Sync + 'static
        {
            self.map_err(|failure| {
                let context = f(&failure);
                failure.context(context)
            })
        }
    }
}
