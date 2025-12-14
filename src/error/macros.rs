#![allow(clippy::single_component_path_imports)]

macro_rules! error {
    ($err:expr, $fmt:expr, $($arg:expr),* $(,)?) => {{
		use ::tracing;
		let err: $crate::error::BotError = $err;
		let enum_variant = format!("{err:?}");
		let cause = format!($fmt, $($arg,)*);
		tracing::error!("{enum_variant}: {cause}");
		err
	}};
    ($err:expr, $cause:expr $(,)?) => {{
		use ::tracing;
		let err: $crate::error::BotError = $err;
		let enum_variant = format!("{err:?}");
		let cause = format!($cause);
		tracing::error!("{enum_variant}: {cause}");
		err
	}};
	($err:expr $(,)?) => {{
		use ::tracing;
		let err: $crate::error::BotError = $err;
		tracing::error!("{err:?}");
		err
	}};
}
pub(crate) use error;

macro_rules! error_macro {
	($name:ident $error:expr) => {
		$crate::error::macros::error_macro!(@$name, $error, with $);
	};

	(@$name:ident, $error:expr, with $dollar:tt) => {
		#[allow(unused_macros)]
		macro_rules! $name {
			() => {
				$error
			};
			($dollar($dollar arg:expr),* $dollar(,)?) => {
				$dollar crate::error::macros::error!($error, $dollar($dollar arg,)*)
			};
		}
	};
}
pub(crate) use error_macro;