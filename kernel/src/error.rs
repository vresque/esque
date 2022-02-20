pub type Result<T, E = UnixError> = core::result::Result<T, E>;

/// # Error
/// A wrapper around an error type
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UnixError(i32);

impl UnixError {
    pub const fn new(code: i32) -> Self {
        Self(code)
    }
    pub fn text(&self) -> &str {
        ERROR_TEXTS
            .get(self.0 as usize)
            .map(|&str| str)
            .unwrap_or("Bad Error Number")
    }

    /// # Encode
    /// Encodes the code into an errno
    pub fn encode<T>(result: Result<T>) -> i32
    where
        T: Into<i32>,
    {
        match result {
            Ok(good) => good.into(),
            Err(bad) => -(bad.0 as i32),
        }
    }

    /// # Decode
    /// Decodes an error code into a result
    pub fn decode<T>(code: i32) -> Result<T>
    where
        T: From<i32>,
    {
        if code <= 0 && -code <= ERROR_TEXTS.len() as i32 {
            /* Error Code */
            Err(UnixError::new(-code))
        } else {
            /* No Error Code */
            Ok(T::from(code))
        }
    }
}

impl core::fmt::Display for UnixError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}", self.text())?;
        Ok(())
    }
}

enumtastic::enum_with_options! {
    pub enum Error: UnixError => {
        OperationNotPermitted = UnixError::new(ErrorCode::EPERM),
        NoSuchFileOrDirectory = UnixError::new(ErrorCode::ENOENT),
        NoSuchProcess = UnixError::new(ErrorCode::ESRCH),
        InterruptedSyscall = UnixError::new(ErrorCode::EINTR),
        IOError = UnixError::new(ErrorCode::EIO),
        NoSuchDeviceOrAddress= UnixError::new(ErrorCode::ENXIO),
        ArgumentListTooLong = UnixError::new(ErrorCode::E2BIG),
        ExecFormatError = UnixError::new(ErrorCode::ENOEXEC),
        BadFileNumber = UnixError::new(ErrorCode::EBADF),
        NoChildProcess = UnixError::new(ErrorCode::ECHILD),
        TryAgain = UnixError::new(ErrorCode::EAGAIN),
        OutOfMemory = UnixError::new(ErrorCode::ENOMEM),
        PermissionDenied = UnixError::new(ErrorCode::EACCES),
        BadFault = UnixError::new(ErrorCode::EFAULT),
        BlockDeviceRequired = UnixError::new(ErrorCode::ENOTBLK),
        DeviceOrResourceBusy = UnixError::new(ErrorCode::EBUSY),
        FileExists = UnixError::new(ErrorCode::EEXIST),
        CrossDeviceLink = UnixError::new(ErrorCode::EXDEV),
        NoSuchDevice = UnixError::new(ErrorCode::ENODEV),
        NotADirectory = UnixError::new(ErrorCode::ENOTDIR),
        IsADirectory = UnixError::new(ErrorCode::EISDIR),
        InvalidArgument = UnixError::new(ErrorCode::EINVAL),
        FileTableOverflow = UnixError::new(ErrorCode::ENFILE),
        TooManyOpenFiles = UnixError::new(ErrorCode::EMFILE),
        NotTTY = UnixError::new(ErrorCode::ENOTTY),
        TextFileBusy = UnixError::new(ErrorCode::ETXTBSY),
        FileTooLarge = UnixError::new(ErrorCode::EFBIG),
        NoSpaceLeftOnDevice = UnixError::new(ErrorCode::ENOSPC),
        IllegalSeek = UnixError::new(ErrorCode::ESPIPE),
        ReadOnlyFileSystem = UnixError::new(ErrorCode::EROFS),
        TooManyLinks = UnixError::new(ErrorCode::EMLINK),
        BrokenPipe = UnixError::new(ErrorCode::EPIPE),
        MathArgumentOutOfDomainOfFunction = UnixError::new(ErrorCode::EDOM),
        MathResultNotRepresentable = UnixError::new(ErrorCode::ERANGE),
        ResourceDeadlockWouldOccur = UnixError::new(ErrorCode::EDEADLK),
        FileNameTooLong = UnixError::new(ErrorCode::ENAMETOOLONG),
        NoRecordLocksAvailable = UnixError::new(ErrorCode::ENOSYS),
        DirectoryNotEmpty = UnixError::new(ErrorCode::ENOTEMPTY),
        TooManySymbolicLinks = UnixError::new(ErrorCode::ELOOP),
        OperationWouldBlock = UnixError::new(ErrorCode::EWOULDBLOCK),
        NoMessageOfDesiredType = UnixError::new(ErrorCode::ENOMSG),
        IdentifierRemoved = UnixError::new(ErrorCode::EIDRM),
        ChannelNumberOutOfRange = UnixError::new(ErrorCode::ECHRNG),
        Level2NotSynchronized = UnixError::new(ErrorCode::ECHRNG),
        Level3Halted = UnixError::new(ErrorCode::EL3HLT),
        Level3Reset = UnixError::new(ErrorCode::EL3RST),
        LinkNumberOutOfRange = UnixError::new(ErrorCode::ELNRNG),
        ProtocolDriverNotAttached = UnixError::new(ErrorCode::EUNATCH),
        NoCSIStructureAvailable = UnixError::new(ErrorCode::ENOCSI),
        Level2Halted = UnixError::new(ErrorCode::EL2HLT),
        InvalidExchange = UnixError::new(ErrorCode::EBADE),
        InvalidRequestDescriptor = UnixError::new(ErrorCode::EBADR),
        ExchangeFull = UnixError::new(ErrorCode::EXFULL),
        NoAnode = UnixError::new(ErrorCode::ENOANO),
        InvalidRequestCode = UnixError::new(ErrorCode::EBADRQC),
        InvalidSlot = UnixError::new(ErrorCode::EBADSLT),
        ResourceDeadlockWouldOccur2 = UnixError::new(ErrorCode::EDEADLOCK),
        BadFontFileFormat = UnixError::new(ErrorCode::EBFONT),
        DeviceNotAStream = UnixError::new(ErrorCode::ENOSTR),
        NoDataAvailable = UnixError::new(ErrorCode::ENODATA),
        TimerExpired = UnixError::new(ErrorCode::ETIME),
        OutOfStreamResources = UnixError::new(ErrorCode::ENOSR),
        MachineIsNotOnTheNetwork = UnixError::new(ErrorCode::ENONET),
        PackageNotInstalled = UnixError::new(ErrorCode::ENOPKG),
        ObjectIsRemote = UnixError::new(ErrorCode::EREMOTE),
        LinkHasBeenSevered = UnixError::new(ErrorCode::ENOLINK),
        AdvertiseError = UnixError::new(ErrorCode::EADV),
        SrmountError = UnixError::new(ErrorCode::ESRMNT),
        CommunicationErrorOnSend = UnixError::new(ErrorCode::ECOMM),
        ProtocolError = UnixError::new(ErrorCode::EPROTO),
        MultihopAttempted = UnixError::new(ErrorCode::EMULTIHOP),
        RFSSpecificError = UnixError::new(ErrorCode::EDOTDOT),
        NotADataMessage = UnixError::new(ErrorCode::EBADMSG),
        Overflow = UnixError::new(ErrorCode::EOVERFLOW),
        NameNotUniqueOnNetwork = UnixError::new(ErrorCode::ENOTUNIQ),
        FileDescriptorInBadState = UnixError::new(ErrorCode::EBADFD),
        RemoteAddressChange = UnixError::new(ErrorCode::EREMCHG),
        CannotAccessANeededSharedLibrary = UnixError::new(ErrorCode::ELIBACC),
        AccessingACorruptedSharedLibrary = UnixError::new(ErrorCode::ELIBBAD),
        LibSectionCorrupted = UnixError::new(ErrorCode::ELIBSCN),
        AttemptingToLinkTooManySharedLibraries = UnixError::new(ErrorCode::ELIBMAX),
        CannotExecASharedLibrary = UnixError::new(ErrorCode::ELIBEXEC),
        IllegalByteSequence = UnixError::new(ErrorCode::EILSEQ),
        InteruptedSyscallRestart = UnixError::new(ErrorCode::ERESTART),
        StreamPipeError = UnixError::new(ErrorCode::ESTRPIPE),
        TooManyUsers = UnixError::new(ErrorCode::EUSERS),
        SocketOperationOnNonSocket = UnixError::new(ErrorCode::ENOTSOCK),
        DestinationAddressRequired = UnixError::new(ErrorCode::EDESTADDRREQ),
        MessageTooLong = UnixError::new(ErrorCode::EMSGSIZE),
        ProtocolWrongTypeForSocket = UnixError::new(ErrorCode::EPROTO),
        ProtocolNotAvailable = UnixError::new(ErrorCode::ENOPROTOOPT),
        ProtocolNotSupported = UnixError::new(ErrorCode::ESOCKTNOSUPPORT),
        OperationNotSupportedOnTransportEndpoing = UnixError::new(ErrorCode::EOPNOTSUPP),
        AddressFamilyNotSupportedByProtocol = UnixError::new(ErrorCode::EAFNOSUPPORT),
        AddressAlreadyInUse = UnixError::new(ErrorCode::EADDRINUSE),
        CannotAssignRequestAddress = UnixError::new(ErrorCode::EADDRNOTAVAIL),
        NetworkIsDown = UnixError::new(ErrorCode::ENETDOWN),
        NetworkIsUnreachable = UnixError::new(ErrorCode::ENETUNREACH),
        NetworkDroppedConnectionDueToReset = UnixError::new(ErrorCode::ENETRESET),
        ConnectionAbortedBySoftware = UnixError::new(ErrorCode::ECONNABORTED),
        ConnectionResetByPeer = UnixError::new(ErrorCode::ECONNRESET),
        NoBufferSpaceAvailable = UnixError::new(ErrorCode::ENOBUFS),
        TransportEndpointAlreadyConnected = UnixError::new(ErrorCode::EISCONN),
        TransportEndpoingNotConnected = UnixError::new(ErrorCode::EISCONN),
        EndpoingHasShutdown = UnixError::new(ErrorCode::ESHUTDOWN),
        TooManyReferences = UnixError::new(ErrorCode::ETOOMANYREFS),
        ConnectionTimedOut = UnixError::new(ErrorCode::ETIMEDOUT),
        ConnectionRefused = UnixError::new(ErrorCode::ECONNREFUSED),
        HostIsDown = UnixError::new(ErrorCode::EHOSTDOWN),
        NoRouteToHost = UnixError::new(ErrorCode::EHOSTUNREACH),
        OperationAlreadyInProgress = UnixError::new(ErrorCode::EINPROGRESS),
        StaleNFSFileHandle = UnixError::new(ErrorCode::ESTALE),
        StructureNeedsCleaning = UnixError::new(ErrorCode::EUCLEAN),
        NotAXENIXNamedTypeFile = UnixError::new(ErrorCode::ENOTNAM),
        NoXENIXSemaphoresAvailable = UnixError::new(ErrorCode::EISNAM),
        RemoteIOError = UnixError::new(ErrorCode::EREMOTEIO),
        QuotaExceeded = UnixError::new(ErrorCode::EDQUOT),
        NoMediumFound = UnixError::new(ErrorCode::ENOMEDIUM),
        WrongMediumType = UnixError::new(ErrorCode::EMEDIUMTYPE),
        RequiredKeyNotAvailable = UnixError::new(ErrorCode::ENOKEY),
        KeyHasExpired = UnixError::new(ErrorCode::EKEYEXPIRED),
        KeyHasBeenRevoked = UnixError::new(ErrorCode::EKEYREVOKED),
        KeyWasRejectedByService = UnixError::new(ErrorCode::EKEYREJECTED),
        OwnerDied = UnixError::new(ErrorCode::EOWNERDEAD),
        StateNotRecoverable = UnixError::new(ErrorCode::ENOTRECOVERABLE),
    }
}

enumtastic::const_enum! {
    uncounted pub enum ErrorCode: i32 => {
        EPERM = 1,  /* Operation not permitted */
        ENOENT = 2,  /* No such file or directory */
        ESRCH = 3,  /* No such process */
        EINTR = 4,  /* Interrupted system call */
        EIO = 5,  /* I/O error */
        ENXIO = 6,  /* No such device or address */
        E2BIG = 7,  /* Argument list too long */
        ENOEXEC = 8,  /* Exec format error */
        EBADF = 9,  /* Bad file number */
        ECHILD = 10,  /* No child processes */
        EAGAIN = 11,  /* Try again */
        ENOMEM = 12,  /* Out of memory */
        EACCES = 13,  /* Permission denied */
        EFAULT = 14,  /* Bad address */
        ENOTBLK = 15,  /* Block device required */
        EBUSY = 16,  /* Device or resource busy */
        EEXIST = 17,  /* File exists */
        EXDEV = 18,  /* Cross-device link */
        ENODEV = 19,  /* No such device */
        ENOTDIR = 20,  /* Not a directory */
        EISDIR = 21,  /* Is a directory */
        EINVAL = 22,  /* Invalid argument */
        ENFILE = 23,  /* File table overflow */
        EMFILE = 24,  /* Too many open files */
        ENOTTY = 25,  /* Not a typewriter */
        ETXTBSY = 26,  /* Text file busy */
        EFBIG = 27,  /* File too large */
        ENOSPC = 28,  /* No space left on device */
        ESPIPE = 29,  /* Illegal seek */
        EROFS = 30,  /* Read-only file system */
        EMLINK = 31,  /* Too many links */
        EPIPE = 32,  /* Broken pipe */
        EDOM = 33,  /* Math argument out of domain of func */
        ERANGE = 34,  /* Math result not representable */
        EDEADLK = 35,  /* Resource deadlock would occur */
        ENAMETOOLONG = 36,  /* File name too long */
        ENOLCK = 37,  /* No record locks available */
        ENOSYS = 38,  /* Function not implemented */
        ENOTEMPTY = 39,  /* Directory not empty */
        ELOOP = 40,  /* Too many symbolic links encountered */
        EWOULDBLOCK = 41,  /* Operation would block */
        ENOMSG = 42,  /* No message of desired type */
        EIDRM = 43,  /* Identifier removed */
        ECHRNG = 44,  /* Channel number out of range */
        EL2NSYNC = 45,  /* Level 2 not synchronized */
        EL3HLT = 46,  /* Level 3 halted */
        EL3RST = 47,  /* Level 3 reset */
        ELNRNG = 48,  /* Link number out of range */
        EUNATCH = 49,  /* Protocol driver not attached */
        ENOCSI = 50,  /* No CSI structure available */
        EL2HLT = 51,  /* Level 2 halted */
        EBADE = 52,  /* Invalid exchange */
        EBADR = 53,  /* Invalid request descriptor */
        EXFULL = 54,  /* Exchange full */
        ENOANO = 55,  /* No anode */
        EBADRQC = 56,  /* Invalid request code */
        EBADSLT = 57,  /* Invalid slot */
        EDEADLOCK = 58, /* Resource deadlock would occur */
        EBFONT = 59,  /* Bad font file format */
        ENOSTR = 60,  /* Device not a stream */
        ENODATA = 61,  /* No data available */
        ETIME = 62,  /* Timer expired */
        ENOSR = 63,  /* Out of streams resources */
        ENONET = 64,  /* Machine is not on the network */
        ENOPKG = 65,  /* Package not installed */
        EREMOTE = 66,  /* Object is remote */
        ENOLINK = 67,  /* Link has been severed */
        EADV = 68,  /* Advertise error */
        ESRMNT = 69,  /* Srmount error */
        ECOMM = 70,  /* Communication error on send */
        EPROTO = 71,  /* Protocol error */
        EMULTIHOP = 72,  /* Multihop attempted */
        EDOTDOT = 73,  /* RFS specific error */
        EBADMSG = 74,  /* Not a data message */
        EOVERFLOW = 75,  /* Value too large for defined data type */
        ENOTUNIQ = 76,  /* Name not unique on network */
        EBADFD = 77,  /* File descriptor in bad state */
        EREMCHG = 78,  /* Remote address changed */
        ELIBACC = 79,  /* Can not access a needed shared library */
        ELIBBAD = 80,  /* Accessing a corrupted shared library */
        ELIBSCN = 81,  /* .lib section in a.out corrupted */
        ELIBMAX = 82,  /* Attempting to link in too many shared libraries */
        ELIBEXEC = 83,  /* Cannot exec a shared library directly */
        EILSEQ = 84,  /* Illegal byte sequence */
        ERESTART = 85,  /* Interrupted system call should be restarted */
        ESTRPIPE = 86,  /* Streams pipe error */
        EUSERS = 87,  /* Too many users */
        ENOTSOCK = 88,  /* Socket operation on non-socket */
        EDESTADDRREQ = 89,  /* Destination address required */
        EMSGSIZE = 90,  /* Message too long */
        EPROTOTYPE = 91,  /* Protocol wrong type for socket */
        ENOPROTOOPT = 92,  /* Protocol not available */
        EPROTONOSUPPORT = 93,  /* Protocol not supported */
        ESOCKTNOSUPPORT = 94,  /* Socket type not supported */
        EOPNOTSUPP = 95,  /* Operation not supported on transport endpoint */
        EPFNOSUPPORT = 96,  /* Protocol family not supported */
        EAFNOSUPPORT = 97,  /* Address family not supported by protocol */
        EADDRINUSE = 98,  /* Address already in use */
        EADDRNOTAVAIL = 99,  /* Cannot assign requested address */
        ENETDOWN = 100, /* Network is down */
        ENETUNREACH = 101, /* Network is unreachable */
        ENETRESET = 102, /* Network dropped connection because of reset */
        ECONNABORTED = 103, /* Software caused connection abort */
        ECONNRESET = 104, /* Connection reset by peer */
        ENOBUFS = 105, /* No buffer space available */
        EISCONN = 106, /* Transport endpoint is already connected */
        ENOTCONN = 107, /* Transport endpoint is not connected */
        ESHUTDOWN = 108, /* Cannot send after transport endpoint shutdown */
        ETOOMANYREFS = 109, /* Too many references: cannot splice */
        ETIMEDOUT = 110, /* Connection timed out */
        ECONNREFUSED = 111, /* Connection refused */
        EHOSTDOWN = 112, /* Host is down */
        EHOSTUNREACH = 113, /* No route to host */
        EALREADY = 114, /* Operation already in progress */
        EINPROGRESS = 115, /* Operation now in progress */
        ESTALE = 116, /* Stale NFS file handle */
        EUCLEAN = 117, /* Structure needs cleaning */
        ENOTNAM = 118, /* Not a XENIX named type file */
        ENAVAIL = 119, /* No XENIX semaphores available */
        EISNAM = 120, /* Is a named type file */
        EREMOTEIO = 121, /* Remote I/O error */
        EDQUOT = 122, /* Quota exceeded */
        ENOMEDIUM = 123, /* No medium found */
        EMEDIUMTYPE = 124, /* Wrong medium type */
        ECANCELED = 125, /* Operation Canceled */
        ENOKEY = 126, /* Required key not available */
        EKEYEXPIRED = 127, /* Key has expired */
        EKEYREVOKED = 128, /* Key has been revoked */
        EKEYREJECTED = 129, /* Key was rejected by service */
        EOWNERDEAD = 130, /* Owner died */
        ENOTRECOVERABLE = 131, /* State not recoverable */
    }

    impl {}
}

pub static ERROR_TEXTS: [&'static str; 132] = [
    "Success",
    "Operation not permitted",
    "No such file or directory",
    "No such process",
    "Interrupted system call",
    "I/O error",
    "No such device or address",
    "Argument list too long",
    "Exec format error",
    "Bad file number",
    "No child processes",
    "Try again",
    "Out of memory",
    "Permission denied",
    "Bad address",
    "Block device required",
    "Device or resource busy",
    "File exists",
    "Cross-device link",
    "No such device",
    "Not a directory",
    "Is a directory",
    "Invalid argument",
    "File table overflow",
    "Too many open files",
    "Not a typewriter",
    "Text file busy",
    "File too large",
    "No space left on device",
    "Illegal seek",
    "Read-only file system",
    "Too many links",
    "Broken pipe",
    "Math argument out of domain of func",
    "Math result not representable",
    "Resource deadlock would occur",
    "File name too long",
    "No record locks available",
    "Function not implemented",
    "Directory not empty",
    "Too many symbolic links encountered",
    "Operation would block",
    "No message of desired type",
    "Identifier removed",
    "Channel number out of range",
    "Level 2 not synchronized",
    "Level 3 halted",
    "Level 3 reset",
    "Link number out of range",
    "Protocol driver not attached",
    "No CSI structure available",
    "Level 2 halted",
    "Invalid exchange",
    "Invalid request descriptor",
    "Exchange full",
    "No anode",
    "Invalid request code",
    "Invalid slot",
    "Resource deadlock would occur",
    "Bad font file format",
    "Device not a stream",
    "No data available",
    "Timer expired",
    "Out of streams resources",
    "Machine is not on the network",
    "Package not installed",
    "Object is remote",
    "Link has been severed",
    "Advertise error",
    "Srmount error",
    "Communication error on send",
    "Protocol error",
    "Multihop attempted",
    "RFS specific error",
    "Not a data message",
    "Value too large for defined data type",
    "Name not unique on network",
    "File descriptor in bad state",
    "Remote address changed",
    "Can not access a needed shared library",
    "Accessing a corrupted shared library",
    ".lib section in a.out corrupted",
    "Attempting to link in too many shared libraries",
    "Cannot exec a shared library directly",
    "Illegal byte sequence",
    "Interrupted system call should be restarted",
    "Streams pipe error",
    "Too many users",
    "Socket operation on non-socket",
    "Destination address required",
    "Message too long",
    "Protocol wrong type for socket",
    "Protocol not available",
    "Protocol not supported",
    "Socket type not supported",
    "Operation not supported on transport endpoint",
    "Protocol family not supported",
    "Address family not supported by protocol",
    "Address already in use",
    "Cannot assign requested address",
    "Network is down",
    "Network is unreachable",
    "Network dropped connection because of reset",
    "Software caused connection abort",
    "Connection reset by peer",
    "No buffer space available",
    "Transport endpoint is already connected",
    "Transport endpoint is not connected",
    "Cannot send after transport endpoint shutdown",
    "Too many references, cannot splice",
    "Connection timed out",
    "Connection refused",
    "Host is down",
    "No route to host",
    "Operation already in progress",
    "Operation now in progress",
    "Stale NFS file handle",
    "Structure needs cleaning",
    "Not a XENIX named type file",
    "No XENIX semaphores available",
    "Is a named type file",
    "Remote I/O error",
    "Quota exceeded",
    "No medium found",
    "Wrong medium type",
    "Operation Canceled",
    "Required key not available",
    "Key has expired",
    "Key has been revoked",
    "Key was rejected by service",
    "Owner died",
    "State not recoverable",
];
