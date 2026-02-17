pub use lightning::events::ClosureReason;
pub use lightning::types::payment::PaymentHash;
pub use lightning::types::string::UntrustedString;

use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;

use std::fmt;
use std::str::FromStr;

uniffi::include_scaffolding!("test");

pub fn return_reason() -> TypeHoldingClosureReason {
	TypeHoldingClosureReason::Variant { reason: ClosureReason::FundingTimedOut }
}

pub enum TypeHoldingClosureReason {
	Variant { reason: ClosureReason },
}

impl UniffiCustomTypeConverter for PaymentHash {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(hash) = Sha256::from_str(&val) {
			Ok(PaymentHash(hash.to_byte_array()))
		} else {
			Err(Error::SomeError.into())
		}
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		Sha256::from_slice(&obj.0).unwrap().to_string()
	}
}

impl UniffiCustomTypeConverter for UntrustedString {
	type Builtin = String;
	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		Ok(UntrustedString(val))
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

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
