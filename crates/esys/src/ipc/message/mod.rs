pub mod numeric;
pub mod ptr;

trait IsMessageTooBig {
    fn too_big();
}

#[macro_export]
macro_rules! assert_msg_size_is_correct {
    ($stru:ident) => {
        static_assertions::assert_eq_size!([u8; 56], $stru);
    };
}

#[macro_export]
macro_rules! implement_message_struct {
    (
        $(#[$attributes_of_struct:meta])*
        $visi:vis struct $name_of_struct:ident {
            $(
                $(#[$attributes_of_field:meta])*
                $vis_of_field:vis $name_of_field:ident : $type_of_field:ty,
            )*
        }
    ) => {
        $(#[$attributes_of_struct])*
        #[derive(Eq, PartialEq, Copy, Clone, Debug)]
        #[allow(unused)]
        #[repr(C)]
        $visi struct $name_of_struct {
            $(
                $(#[$attributes_of_field])*
                $vis_of_field $name_of_field: $type_of_field,
            )*
        }
        crate::assert_msg_size_is_correct!($name_of_struct);

        impl $name_of_struct {
            pub fn new(
                $(
                    $name_of_field: $type_of_field,
                )*
            ) -> Self {
                Self {
                    $(
                    $name_of_field,
                    )*
                }
            }
        }
    }
}
