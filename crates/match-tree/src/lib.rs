#![no_std]

#[macro_export]
macro_rules! match_tree {
    (
        grand = $grand_matcher:expr => major = $major_matcher:expr => minor = $minor_matcher:expr =>
        config: { grand-default = $final_grand:expr ; major-default = $final_major:expr ; minor-default = $final_minor:expr } => {
        $(
            $grand_match:tt {
                $(
                    $major_match:tt {
                        $(
                            $minor_match:tt = $minor_result:expr,
                        )*
                    },
                )*
        },
    )*
    }
    ) => {
        match $grand_matcher {
            $(
                $grand_match => {
                    match $major_matcher {
                        $(
                            $major_match => {
                                match $minor_matcher {
                                    $(
                                        $minor_match => { $minor_result },
                                    )*
                                    _ => { $final_minor }
                                }
                            },
                        )*
                        _ => { $final_major }
                    }
                }
            )*
            _ => { $final_grand }
        }
    };

    (
        major = $major_matcher:expr => minor = $minor_matcher:expr =>
        config: { major-default = $final_major:expr ; minor-default = $final_minor:expr } => {$(
            $major_match:tt {
                $(
                    $minor_match:tt = $minor_result:expr,
                )*
            },
        )*
    }
    ) => {
        match $major_matcher {
            $(
                $major_match => {
                    match $minor_matcher {
                        $(
                            $minor_match => { $minor_result },
                        )*
                        _ => { $final_minor }
                    }
                },
            )*
            _ => { $final_major }
        }
    };

}
