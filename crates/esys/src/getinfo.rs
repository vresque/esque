enumtastic::const_enum! {
    pub enum SysGetInfoValues: u32 => {
        GetKernelInfo = 0,
        GetImage = 1,
        GetProcessTable = 2,
        GetRandomness = 3,
        GetMonitorParameters = 4,
        GetKernelEnvironment = 5,
        GetIrqHooks = 6,
        GetPriviligesTable = 8,
        GetKernelAddresses = 9,
        GetSchedulerInfo = 10,
        GetProcessSlotIfGivenProcess = 11,
        GetMachineInfo = 12,
        GetLockTiming = 13,
        GET_INFO_NONE = 14,
        GetLoadInfo = 15,
        GetIrqMascs = 16,
        GetPrivilegeStructure = 17,
    }
    impl {}
}
