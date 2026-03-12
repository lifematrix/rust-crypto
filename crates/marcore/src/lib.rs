pub mod floatx;
pub mod formatx;
pub mod optionx;

pub use floatx::FloatX;
pub use optionx::OptionExt;

#[macro_export]
macro_rules! pkg_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}
