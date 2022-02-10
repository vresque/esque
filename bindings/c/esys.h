// Pushes an struct IPCMessage* onto the stack of IPC functions
// returns the index on the IPC Stack
int ipc_push(struct IPCMessage* msg);

// Shortcut for:
// while (msg->is_resolved == EFALSE);
// return msg->ptr
void* ipc_await(struct IPCMessage* msg);
