enumtastic::const_enum! {
    pub enum TarEntryType: u8 => {
        RegularFile = '0' as u8,
        ARegularFile = '\0' as u8,
        Link = '1' as u8,
        Reserved = '2' as u8,
        CharacterSpecial = '3' as u8,
        BlockSpecial = '4' as u8,
        Directory = '5' as u8,
        FifoSpecial = '6' as u8,
        Cont = '7' as u8,
    }

    impl {}
}
