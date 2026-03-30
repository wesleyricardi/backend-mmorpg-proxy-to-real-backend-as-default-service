#[macro_export]
macro_rules! __err_invalid_base {
    ($kind:expr) => {
        $crate::error::AppError::new($kind)
    };

    ($kind:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::error::AppError::new($kind).set_internal_message(
            ::std::format!($fmt $(, $arg)*),
            ::std::file!(),
            ::std::line!(),
        )
    };

    ($kind:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::error::AppError::new($kind)
            .set_cause($cause)
            .set_internal_message(
                ::std::format!($fmt $(, $arg)*),
                ::std::file!(),
                ::std::line!(),
            )
    };
}

#[macro_export]
macro_rules! bad_request {
    () => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::InvalidInput(
                $crate::error::InputErrorOrigin::InternalValidation(
                    "erro de validacao interna: entrada invalida".to_string()
                )
            )
        )
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::InvalidInput(
                $crate::error::InputErrorOrigin::InternalValidation(
                    "erro de validacao interna: entrada invalida".to_string()
                )
            ),
            $($t)*
        )
    };
}

#[macro_export]
macro_rules! input_violation {
    ($domain:expr, $kind:expr $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::InputViolation($domain, $kind))
    };
    ($domain:expr, $kind:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::InputViolation($domain, $kind),
            $fmt $(, $arg)*
        )
    };
    ($domain:expr, $kind:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::InputViolation($domain, $kind),
            $cause,
            $fmt $(, $arg)*
        )
    };
}

#[macro_export]
macro_rules! unexpected_error {
    () => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::UnexpectedError)
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::UnexpectedError, $($t)*)
    };
}

#[macro_export]
macro_rules! invalid_credentials {
    () => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::InvalidCredentials)
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::InvalidCredentials, $($t)*)
    };
}

#[macro_export]
macro_rules! empty_data {
    () => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::EmptyData)
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::EmptyData, $($t)*)
    };
}

#[macro_export]
macro_rules! data_not_found {
    () => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::DataNotFound)
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::DataNotFound, $($t)*)
    };
}

#[macro_export]
macro_rules! permission_denied {
    () => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::PermissionDenied)
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::PermissionDenied, $($t)*)
    };
}

#[macro_export]
macro_rules! conflict {
    ($domain:expr, $kind:expr $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::Conflict($domain, $kind))
    };
    ($domain:expr, $kind:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::Conflict($domain, $kind),
            $fmt $(, $arg)*
        )
    };
    ($domain:expr, $kind:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::Conflict($domain, $kind),
            $cause,
            $fmt $(, $arg)*
        )
    };
    () => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::Conflict(
                $crate::error::Domain::Setting,
                $crate::error::ConflictKind::GenericConflict,
            )
        )
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::Conflict(
                $crate::error::Domain::Setting,
                $crate::error::ConflictKind::GenericConflict,
            ),
            $($t)*
        )
    };
}

#[macro_export]
macro_rules! reference_not_found {
    () => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::ReferenceNotFound)
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::ReferenceNotFound, $($t)*)
    };
}

#[macro_export]
macro_rules! reference_invalid {
    () => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::ReferenceInvalid)
    };
    ($($t:tt)*) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::ReferenceInvalid, $($t)*)
    };
}

#[macro_export]
macro_rules! unique_violation {
    ($field:expr $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::UniqueViolation(
                ::std::convert::Into::<::std::string::String>::into($field)
            )
        )
    };
    ($field:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::UniqueViolation(
                ::std::convert::Into::<::std::string::String>::into($field)
            ),
            $fmt $(, $arg)*
        )
    };
    ($field:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::UniqueViolation(
                ::std::convert::Into::<::std::string::String>::into($field)
            ),
            $cause,
            $fmt $(, $arg)*
        )
    };
}

#[macro_export]
macro_rules! rule_violation {
    ($domain:expr, $rule:expr $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::RuleViolation($domain, $rule))
    };
    ($domain:expr, $rule:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::RuleViolation($domain, $rule),
            $fmt $(, $arg)*
        )
    };
    ($domain:expr, $rule:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::RuleViolation($domain, $rule),
            $cause,
            $fmt $(, $arg)*
        )
    };
}

#[macro_export]
macro_rules! io_error {
    ($kind:expr $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::Io($kind))
    };
    ($kind:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::Io($kind), $fmt $(, $arg)*)
    };
    ($kind:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::Io($kind),
            $cause,
            $fmt $(, $arg)*
        )
    };
}

#[macro_export]
macro_rules! parse_error {
    ($kind:expr $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::Parse($kind))
    };
    ($kind:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::Parse($kind), $fmt $(, $arg)*)
    };
    ($kind:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::Parse($kind),
            $cause,
            $fmt $(, $arg)*
        )
    };
}

#[macro_export]
macro_rules! state_error {
    ($kind:expr $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::State($kind))
    };
    ($kind:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::State($kind), $fmt $(, $arg)*)
    };
    ($kind:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::State($kind),
            $cause,
            $fmt $(, $arg)*
        )
    };
}

#[macro_export]
macro_rules! protocol_error {
    ($kind:expr $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::Protocol($kind))
    };
    ($kind:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!($crate::error::ErrorKind::Protocol($kind), $fmt $(, $arg)*)
    };
    ($kind:expr, $cause:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__err_invalid_base!(
            $crate::error::ErrorKind::Protocol($kind),
            $cause,
            $fmt $(, $arg)*
        )
    };
}
