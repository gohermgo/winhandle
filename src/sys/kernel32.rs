use crate::sys::{BOOL, HANDLE};

#[link(name = "Kernel32.dll")]
unsafe extern "system" {
    fn GetLastError() -> u32;
    fn CloseHandle(hobject: HANDLE) -> BOOL;
}

/// # Safety
/// Caller responsible for making sure the handle is valid
#[inline]
pub unsafe fn close_raw_handle_unchecked(raw_handle: HANDLE) -> Result<(), u32> {
    let succeeded = unsafe { CloseHandle(raw_handle) };

    if !succeeded.into_bool() {
        Err(unsafe { GetLastError() })
    } else {
        Ok(())
    }
}

#[inline]
pub fn close_raw_handle(raw_handle: HANDLE) -> Result<(), u32> {
    use crate::sys::ERROR_INVALID_HANDLE;

    if raw_handle.is_invalid() {
        Err(ERROR_INVALID_HANDLE)
    } else {
        unsafe { close_raw_handle_unchecked(raw_handle) }
    }
}
