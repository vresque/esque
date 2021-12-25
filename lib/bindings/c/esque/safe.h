#ifndef _ESQUE_LIB_SAFE_H
#define _ESQUE_LIB_SAFE_H

typedef long unsigned int __esq_usize;
#include <stdint.h>
struct Safe {
    uint64_t kind;
    __esq_usize size;
    uint8_t ty;
    __esq_usize elem;
    uint64_t address;
};

enum SyscallPtrType {
    SYSCALL_PTR_TYPE_VOID = 0,
    SYSCALL_PTR_TYPE_INT = 1,
    SYSCALL_PTR_TYPE_FLOAT = 2,
    SYSCALL_PTR_TYPE_FILE_HANDLE = 3,
};

enum SafeType {
    SAFETYPE_SOME = 0,
    SAFETYPE_NONE = 1,
    SAFETYPE_SOME_ARRAY = 2,
};

struct Safe safe(void* ptr, uint64_t kind, __esq_usize  size, uint8_t ptr_type) {
    return (Safe){
        .kind = kind,
        .size = size,
        .ty = ptr_type,
        .elem = 1,
        .address = (uint64_t)ptr,
    };
}

struct Safe STDIN = safe((void*)0, SAFETYPE_SOME, sizeof(int), SYSCALL_PTR_TYPE_FILE_HANDLE);
struct Safe STDOUT = safe((void*)1, SAFETYPE_SOME, sizeof(int), SYSCALL_PTR_TYPE_FILE_HANDLE);
struct Safe STDERR = safe((void*)2, SAFETYPE_SOME, sizeof(int), SYSCALL_PTR_TYPE_FILE_HANDLE);

#endif