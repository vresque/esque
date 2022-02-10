# HAL IPC
## Hardware Abstraction Layer Inter Process Communication

The IPC that all *system space* applications may use is simple. All *system space* applications receive a `*mut EProcessHandle` on startup. This Process Handle contains the address at which all ipc's must be written to. 

The IPC looks like this:
```
---
IPCHeader (Fields: slots (Array of bools that are true if slot is occupied, false if not))
---
IPCMessage 1
---
IPCMessage 2
---
...
---
IPCMessage 255
---
```

IPC's may be send using the following functions (here, C is used)
```c
// Pushes an struct IPCMessage* onto the stack of IPC functions
// returns the index on the IPC Stack
int ipc_push(struct IPCMessage* msg);
// Shortcut for:
// while (msg->is_resolved == EFALSE);
// return msg->ptr
void* ipc_await(struct IPCMessage* msg);

// How to resolve an IPC yourself:
// msg->is_resolved = ETRUE
// The Kernel will clear this slot the next time it visits it