#![doc = include_str!("../README.md")]

/// Trait for deep flattening Options.
pub trait OptionDeepFlatten<T> {
    /// Convert nested Options e.g. `Option<Option<Option<T>>>` into Option<T>.
    /// Can convert up to 32 Options.
    fn deep_flatten(self) -> Option<T>;
}

macro_rules! __impl_option {
    ($_ignored:ident) => {};
    ($_ignored:ident $($t:ident)+) => {
        impl<T> OptionDeepFlatten<T> for __impl_option!(@nest_option $($t)+) {
            fn deep_flatten(self) -> Option<T> {
                match self {
                    __impl_option!(@nest_some(inner) $($t)+) => Some(inner),
                    _ => None,
                }
            }
        }
        __impl_option!($($t)*);
    };

    (@nest_option $_ignored:ident) => { Option<T> };
    (@nest_option $_ignored:ident $($t:ident)+) => {
        Option<__impl_option!(@nest_option $($t)+)>
    };
    (@nest_some($dst:ident) $($_ignored:ident)?) => { Some($dst) };
    (@nest_some($dst:ident) $_ignored:ident $($t:ident)+) => {
        Some(__impl_option!(@nest_some($dst) $($t)+))
    };
}

// Impl for up to 32 Options.
__impl_option!(IGNORED
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option 
    Option
);
