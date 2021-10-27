pub mod error {
    //! todo
    use thiserror::Error;
    #[cfg(feature = "json")]
    #[doc(cfg(feature = "json"))]
    pub use crate::backend::JsonError;
    /// todo
    pub enum ChartError {
        /// todo
        #[cfg(feature = "json")]
        #[doc(cfg(feature = "json"))]
        #[error(transparent)]
        Json(#[from] JsonError),
        /// todo
        #[error(transparent)]
        Custom(Box<dyn std::error::Error + Send + Sync>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ChartError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ChartError::Json(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Json");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ChartError::Custom(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Custom");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for ChartError {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;
            #[allow(deprecated)]
            match self {
                ChartError::Json { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                ChartError::Custom { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for ChartError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                ChartError::Json(_0) => std::fmt::Display::fmt(_0, __formatter),
                ChartError::Custom(_0) => std::fmt::Display::fmt(_0, __formatter),
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<JsonError> for ChartError {
        #[allow(deprecated)]
        fn from(source: JsonError) -> Self {
            ChartError::Json { 0: source }
        }
    }
}
