//! A field-less enum whose serde name and `&'static str` slug come from one list.

/// Declares a field-less enum with `Serialize` and `Deserialize` and a `const fn slug`. Each
/// variant carries its serde name once (`User = "user"`); the macro writes both the
/// `#[serde(rename)]` and the `slug` arm, so the two cannot differ. A span attribute value must be a
/// constant, which serde cannot give.
macro_rules! slug_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident = $slug:literal ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$vmeta])*
                #[serde(rename = $slug)]
                $variant,
            )+
        }

        impl $name {
            /// The slug for a span attribute: the same text as its serde form, as a `&'static
            /// str`.
            pub const fn slug(self) -> &'static str {
                match self {
                    $( Self::$variant => $slug, )+
                }
            }
        }
    };
}

pub(crate) use slug_enum;
