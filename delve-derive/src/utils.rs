macro_rules! unwrap_attrs {
    (
        $($ident:ident)::* ($($args:tt)*) ?
    ) => {
        match $($ident)::*($($args)*) {
            Ok(a) => a,
            Err(e) => return Err(e.into())
        }
    };

    (
        $($ident:ident)::* ($($args:tt)*)
    ) => {
        match $($ident)::*($($args)*) {
            Ok(a) => a,
            Err(e) => return e.to_compile_error().into()
        }
    }
}

pub(crate) use unwrap_attrs;

macro_rules! syn_err {
    ($l:literal $(, $a:expr)*) => {
        syn_err!(proc_macro2::Span::call_site(); $l $(, $a)*)
    };
    ($s:expr; $l:literal $(, $a:expr)*) => {
        return Err(syn::Error::new($s, format!($l $(, $a)*)))
    };
}

pub(crate) use syn_err;
