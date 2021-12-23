#![no_std]

#[macro_export]
macro_rules! const_enum {
    (
        $(#[$attributes_of_enum:meta])*
        pub enum $name_of_enum:ident : $integer_type:ty => $(#[$impl_attributes_of_enum:meta])* {
            $(
                $(#[$attributes_of_variant:meta])*
                $name_of_variant:ident = $value_of_variant:expr,
            )*
        }

        impl $(
            $visi:vis fn $nam:ident ($($arg:ident : $typ:ty)*) $blck:block
        )*
    ) => {
        #[allow(non_snake_case)]
        pub mod $name_of_enum {
            $(
                // Allow it to be "Enum-Like" instead of "Const-Like"
                #[allow(non_upper_case_globals)]
                $(#[$attributes_of_variant])*
                pub const $name_of_variant: $integer_type = $value_of_variant;
            )*

            type Me = $integer_type;

            $(
                $visi fn $name_of_fn ($($arg : $typ)*) $blck
            )*
        }
    }
}

#[macro_export]
macro_rules! enum_with_options {
    (
        $(#[$attributes_of_enum:meta])*
        pub enum $name_of_enum:ident : $integer_type:ty => $(#[$impl_attributes_of_enum:meta])* {
            $(
                $(#[$attributes_of_variant:meta])*
                $name_of_variant:ident = $value_of_variant:expr,
            )*
        }
    )
    =>
    {
        // Defining the base structure
        $(#[$attributes_of_enum])*
        #[derive(Eq, PartialEq, Copy, Clone)]
        #[repr(transparent)]
        pub struct $name_of_enum(pub $integer_type);

        #[allow(unused)]
        $(#[$impl_attributes_of_enum])*
        impl $name_of_enum {
            // Each variant is a const with the value of $name_of_enum and inner of value
            $(
                // Allow it to be "Enum-Like" instead of "Const-Like"
                #[allow(non_upper_case_globals)]
                $(#[$attributes_of_variant])*
                pub const $name_of_variant: $name_of_enum = $name_of_enum($value_of_variant);
            )*
        }

        // Implementing Display
        impl ::core::fmt::Debug for $name_of_enum {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    // Each of the CONSTANTS gets formatted with their name
                    // (For example, if we take MemoryType)
                    // This will expand to this
                    // ...
                    // match *self {
                    //  MemoryType::LoaderData /* ($name_of_enum::$name_of_variant) */ => write!(f, stringify!(LoaderData))
                    //}
                    $(
                        $name_of_enum::$name_of_variant => write!(f, stringify!($name_of_variant)),
                    )*

                    // Unknown type
                    $name_of_enum(custom_value) => {
                        write!(f, "{}::custom({})", stringify!($name_of_enum), custom_value)
                    }
                }
            }
        }
    }
}
