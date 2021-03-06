//! A crate for providing native and familiar support for the Windows Runtime for Rust developers.
//!
//! To use, start by importing the types that you need.
//!
//! ```rust
//! use winrt::import;
//!
//! import!(
//!     dependencies
//!         os
//!     types
//!         windows::foundation::Uri
//! );
//!
//! // Make use of any WinRT APIs as needed.
//! // For example, here is an example of using the Windows.Foundation.Uri class:
//! fn main() -> winrt::Result<()> {
//!     use windows::foundation::Uri;
//!
//!     let uri = Uri::create_uri("http://kennykerr.ca")?;
//!     println!("domain: {}", uri.domain()?);
//!     println!("port: {}", uri.port()?);
//!     println!("string: {}", uri.to_string()?);
//!
//!     Ok(())
//! }
//! ```
//!
//! The actual APIs available to you will depend on what WinRT modules you are using.
//! The documentation found here is relevant for core functionality for WinRT, but
//! ultimately the code you will interact with most will be generated by this crate from
//! metadata. For example, documentation on using the `window::foundation::Uri` type
//! above, can be found [here](https://docs.microsoft.com/en-us/uwp/api/Windows.Foundation.Uri?view=winrt-19041). Of
//! course, when using these APIs from Rust, you will have to remember to translate CamelCase to snake_case as is
//! the convention in Rust.
//!
//! This program will print the following output:
//!
//! ```text
//! domain: kennykerr.ca
//! port: 80
//! string: http://kennykerr.ca/
//! ```

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod abi_transferable;
mod activation_factory;
mod agile_object;
mod array;
mod com;
mod error;
mod factory;
mod guid;
mod hstring;
mod object;
mod param;
mod runtime;
mod runtime_name;
mod runtime_type;

#[doc(hidden)]
pub use abi_transferable::AbiTransferable;
#[doc(hidden)]
pub use activation_factory::IActivationFactory;
#[doc(hidden)]
pub use agile_object::IAgileObject;
pub use array::Array;
#[doc(hidden)]
pub use com::*;
pub use error::*;
#[doc(hidden)]
pub use factory::factory;
#[doc(hidden)]
pub use guid::Guid;
pub use hstring::HString;
pub use object::Object;
#[doc(hidden)]
pub use param::Param;
#[doc(hidden)]
pub use runtime::*;
#[doc(hidden)]
pub use runtime_name::RuntimeName;
#[doc(hidden)]
pub use runtime_type::RuntimeType;
pub use winrt_macros::{build, import};

#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;
