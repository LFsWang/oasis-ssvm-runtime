pub mod codes {
	// NOTE [ToDr] Codes from [-32099, -32000]
	pub const UNSUPPORTED_REQUEST: i64 = -32000;
	pub const NO_WORK: i64 = -32001;
	pub const NO_AUTHOR: i64 = -32002;
	pub const NO_NEW_WORK: i64 = -32003;
	pub const NO_WORK_REQUIRED: i64 = -32004;
	pub const CANNOT_SUBMIT_WORK: i64 = -32005;
	pub const CANNOT_SUBMIT_BLOCK: i64 = -32006;
	pub const UNKNOWN_ERROR: i64 = -32009;
	pub const TRANSACTION_ERROR: i64 = -32010;
	pub const EXECUTION_ERROR: i64 = -32015;
	pub const EXCEPTION_ERROR: i64 = -32016;
	pub const DATABASE_ERROR: i64 = -32017;
	#[cfg(any(test, feature = "accounts"))]
	pub const ACCOUNT_LOCKED: i64 = -32020;
	#[cfg(any(test, feature = "accounts"))]
	pub const PASSWORD_INVALID: i64 = -32021;
	pub const ACCOUNT_ERROR: i64 = -32023;
	pub const PRIVATE_ERROR: i64 = -32024;
	pub const REQUEST_REJECTED: i64 = -32040;
	pub const REQUEST_REJECTED_LIMIT: i64 = -32041;
	pub const REQUEST_NOT_FOUND: i64 = -32042;
	pub const ENCRYPTION_ERROR: i64 = -32055;
	#[cfg(any(test, feature = "accounts"))]
	pub const ENCODING_ERROR: i64 = -32058;
	pub const FETCH_ERROR: i64 = -32060;
	pub const NO_LIGHT_PEERS: i64 = -32065;
	pub const NO_PEERS: i64 = -32066;
	pub const DEPRECATED: i64 = -32070;
	pub const EXPERIMENTAL_RPC: i64 = -32071;
	pub const CANNOT_RESTART: i64 = -32080;
}
