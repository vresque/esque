use super::gdt::GdtEntryType;
use super::gdt::Ring;
use super::segment::Segment;
use super::tss::ProcessorControl;
use crate::arch::tss::Tss;
use crate::info;
use crate::{arch::interrupts::register::Registers, syscall};
use core::arch::asm;

#[no_mangle]
pub unsafe extern "C" fn syscall_dispatcher(regs: *mut Registers) {
    info!("Called syscall!");
    let regs = &mut *regs;
    // Return code is in rax
    regs.rax = {
        syscall::syscall(
            regs.rax, regs.rdi, regs.rsi, regs.rdx, regs.r10, regs.r8, regs.r9, regs.rbp, regs,
        )
    }
}

// The following code was copied from the redox kernel: https://gitlab.redox-os.org/redox-os/kernel
// MIT License
//
// Copyright (c) 2017 Jeremy Soller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// Source of this assembly code: https://github.com/redox-os/kernel/blob/master/src/arch/x86_64/interrupt/syscall.rs
// Lines 64-139
#[naked]
#[no_mangle]
pub unsafe extern "C" fn syscall_handler() {
    asm!(
        "
        swapgs                    // Set gs segment to TSS
        mov gs:[{sp}], rsp        // Save userspace stack pointer
        mov rsp, gs:[{ksp}]       // Load kernel stack pointer
        push QWORD PTR {ss_sel}   // Push fake userspace SS (resembling iret frame)
        push QWORD PTR gs:[{sp}]  // Push userspace rsp
        push r11                  // Push rflags
        push QWORD PTR {cs_sel}   // Push fake CS (resembling iret stack frame)
        push rcx                  // Push userspace return pointer

        push rax
        push rcx
        push rdx
        push rdi
        push rsi
        push r8
        push r9
        push r10
        push r11
        push rbx
        push rbp
        push r12
        push r13
        push r14
        push r15
        mov rdi, rsp
        call syscall_dispatcher
        pop r15
        pop r14
        pop r13
        pop r12
        pop rbp
        pop rbx
        pop r11
        pop r10
        pop r9
        pop r8
        pop rsi
        pop rdi
        pop rdx
        pop rcx
        pop rax


        // Return
        //
        // We must test whether RCX is canonical; if it is not when running sysretq, the consequences
        // can be fatal.
        //
        // See https://xenproject.org/2012/06/13/the-intel-sysret-privilege-escalation/.
        //
        // This is not just theoretical; ptrace allows userspace to change RCX (via RIP) of target
        // processes.

            // Set ZF iff forbidden bits 63:47 (i.e. the bits that must be sign extended) of the pushed
            // RCX are set.
            test DWORD PTR [rsp + 4], 0xFFFF8000
            // If ZF was set, i.e. the address was invalid higher-half, so jump to the slower iretq and
            // handle the error without being able to execute attacker-controlled code!
            jnz 1f
            // Otherwise, continue with the fast sysretq.
            pop rcx                 // Pop userspace return pointer
            add rsp, 8              // Pop fake userspace CS
            pop r11                 // Pop rflags
            pop QWORD PTR gs:[{sp}] // Pop userspace stack pointer
            mov rsp, gs:[{sp}]      // Restore userspace stack pointer
            swapgs                  // Restore gs from TSS to user data
            sysretq                 // Return into userspace; RCX=>RIP,R11=>RFLAGS
    1:
            // Slow iretq
            xor rcx, rcx
            xor r11, r11
            swapgs
            iretq
        ", 
        sp = const(memoffset::offset_of!(ProcessorControl, user_rsp)),
        ss_sel = const(Segment::new(Ring::Ring3, GdtEntryType::UserData).bits()),
        ksp = const((memoffset::offset_of!(ProcessorControl, tss) + memoffset::offset_of!(Tss, rsp))),
        cs_sel = const(Segment::new(Ring::Ring3, GdtEntryType::UserCode).bits()),
        options(noreturn),
    );
}
