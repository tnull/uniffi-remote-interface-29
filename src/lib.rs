pub use lightning::events::ClosureReason;
pub use lightning::types::payment::PaymentHash;
pub use lightning::types::string::UntrustedString;
pub use vss_client::headers::{VssHeaderProvider, VssHeaderProviderError};

use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;

use std::fmt;
use std::str::FromStr;

uniffi::include_scaffolding!("test");

uniffi::custom_type!(PaymentHash, String, {
	remote,
	try_lift: |val| {
		if let Ok(hash) = Sha256::from_str(&val) {
			Ok(PaymentHash(hash.to_byte_array()))
		} else {
			Err(Error::SomeError.into())
		}
	},
	lower: |obj| {
		Sha256::from_slice(&obj.0).unwrap().to_string()
	},
});

uniffi::custom_type!(UntrustedString, String, {
	remote,
	try_lift: |val| {
		Ok(UntrustedString(val))
	},
	lower: |obj| {
		obj.to_string()
	},
});

#[derive(Debug)]
pub enum Error {
	SomeError,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::SomeError => write!(f, "SomeError"),
		}
	}
}

impl std::error::Error for Error {}
