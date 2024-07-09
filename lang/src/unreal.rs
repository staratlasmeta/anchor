use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use std::alloc::{GlobalAlloc, Layout};
use std::ffi::c_void;
use std::ptr::{null_mut, slice_from_raw_parts_mut, NonNull};
use std::sync::atomic::{AtomicPtr, Ordering};

// extern "C" {
//     // FMemory::Malloc
//     fn rust_malloc(size: usize, alignment: usize) -> *mut c_void;
//     // FMemory::Free
//     fn rust_free(ptr: *mut c_void);
// }

struct AllocatorFunctions {
    malloc: extern "C" fn(usize, usize) -> *mut c_void,
    free: extern "C" fn(*mut c_void),
}
struct Alloc(AtomicPtr<AllocatorFunctions>);

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match NonNull::new(self.0.load(Ordering::SeqCst)) {
            None => panic!("Allocator not set"),
            Some(f) => unsafe { ((*f.as_ptr()).malloc)(layout.size(), layout.align()) as *mut u8 },
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        match NonNull::new(self.0.load(Ordering::SeqCst)) {
            None => panic!("Allocator not set"),
            Some(f) => unsafe { ((*f.as_ptr()).free)(ptr as *mut c_void) },
        }
    }
}

#[global_allocator]
static ALLOC: Alloc = Alloc(AtomicPtr::new(null_mut()));

#[no_mangle]
extern "C" fn set_allocator_functions(
    malloc: extern "C" fn(usize, usize) -> *mut c_void,
    free: extern "C" fn(*mut c_void),
) {
    if ALLOC
        .0
        .compare_exchange(
            null_mut(),
            Box::leak(Box::new(AllocatorFunctions { malloc, free })),
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .is_err()
    {
        panic!("Allocator already set");
    }
}

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
