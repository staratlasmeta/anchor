use std::alloc::{GlobalAlloc, Layout};
use std::ffi::c_void;
use std::ptr::slice_from_raw_parts_mut;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;

extern "C" {
    // FMemory::Malloc
    fn rust_malloc(size: usize, alignment: usize) -> *mut c_void;
    // FMemory::Free
    fn rust_free(ptr: *mut c_void);
}

struct Alloc;

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { rust_malloc(layout.size(), layout.align()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { rust_free(ptr as *mut c_void) }
    }
}

#[global_allocator]
static ALLOC: Alloc = Alloc;

#[repr(C)]
pub struct BetterCInstruction {
    pub program_id: Pubkey,
    /// Metadata describing accounts that should be passed to the program.
    pub accounts: *mut AccountMeta,
    pub accounts_len: usize,
    /// Opaque data passed to the program for its own interpretation.
    pub data: *mut u8,
    pub data_len: usize,
}

impl Drop for BetterCInstruction {
    fn drop(&mut self) {
        unsafe { drop_better_c_instruction(self) }
    }
}

impl From<Instruction> for BetterCInstruction {
    fn from(value: Instruction) -> Self {
        let accounts = Box::leak(value.accounts.into_boxed_slice());
        let data = Box::leak(value.data.into_boxed_slice());
        Self {
            program_id: value.program_id,
            accounts: accounts.as_mut_ptr(),
            accounts_len: accounts.len(),
            data: data.as_mut_ptr(),
            data_len: data.len(),
        }
    }
}

/// # Safety
/// Don't call this thing in rust code
#[no_mangle]
pub unsafe extern "C" fn drop_better_c_instruction(v: &mut BetterCInstruction) {
    unsafe {
        drop(Box::from_raw(slice_from_raw_parts_mut(
            v.accounts,
            v.accounts_len,
        )));
        drop(Box::from_raw(slice_from_raw_parts_mut(v.data, v.data_len)));
    }
}
