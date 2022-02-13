pub const SIGNAL_COUNT: usize = 26;

pub enum SignalAction {
    Ignore,
    Continue,
    Stop,
    Handle(fn()),
}

// https://en.wikipedia.org/wiki/Signal_(IPC)
pub const SIGNAL_TABLE: [SignalAction; SIGNAL_COUNT] = [
    SignalAction::Handle(terminate_core_dump), // SIGABRT
    SignalAction::Handle(terminate),           // SIGALRM
    SignalAction::Handle(terminate_core_dump), // SIGBUS
    SignalAction::Ignore,                      // SIGCHLD
    SignalAction::Continue,                    // SIGCONT
    SignalAction::Handle(terminate_core_dump), // SIGFPE
    SignalAction::Handle(terminate),           // SIGHUP
    SignalAction::Handle(terminate_core_dump), // SIGILL
    SignalAction::Handle(terminate),           // SIGINT
    SignalAction::Handle(terminate),           // SIGKILL
    SignalAction::Handle(terminate),           // SIGPIPE
    SignalAction::Handle(terminate),           // SIGPOLL
    SignalAction::Handle(terminate),           // SIGPROF
    SignalAction::Handle(terminate_core_dump), // SIGQUIT
    SignalAction::Handle(terminate_core_dump), // SIGSEGV
    SignalAction::Stop,                        // SIGSTP
    SignalAction::Stop,                        // SIGTSTP
    SignalAction::Stop,                        // SIGTTOU
    SignalAction::Handle(terminate),           // SIGUSR1
    SignalAction::Handle(terminate),           // SIGUSR2
    SignalAction::Ignore,                      // SIGURG
    SignalAction::Handle(terminate),           // SIGVTALRM
    SignalAction::Handle(terminate_core_dump), // SIGXCPU
    SignalAction::Handle(terminate_core_dump), // SIGXFSZ
    SignalAction::Ignore,                      // SIGWINCH
    SignalAction::Stop,
];

pub fn terminate() {}
pub fn terminate_core_dump() {}
