macro_rules! warn_and_return {
	($fmt:literal $(,)?) => {{
		use anyhow::anyhow;
		use tracing::warn;

		let err = format!($fmt);
		warn!(err);
		anyhow!(err)
	}};
    ($fmt:literal, $($arg:tt)*) => {{
		use anyhow::anyhow;
		use tracing::warn;

		let err = format!($fmt, $($arg)*);
		warn!(err);
		anyhow!(err)
	}};
    ($err:expr $(,)?) => {{
		use anyhow::anyhow;
		use tracing::warn;

		let err = format!("{}", $err);
		warn!(err);
		anyhow!(err)
	}};
}

pub(crate) use warn_and_return;