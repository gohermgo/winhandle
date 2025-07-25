mod kernel32;

#[expect(clippy::upper_case_acronyms)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
struct BOOL(u32);

impl BOOL {
    #[inline]
    pub const fn into_bool(self) -> bool {
        !matches!(self, BOOL(0))
    }
}

impl From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.into_bool()
    }
}

#[expect(clippy::upper_case_acronyms)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HANDLE(pub *mut core::ffi::c_void);

pub const INVALID_HANDLE_VALUE: HANDLE = HANDLE(-1_isize as *mut core::ffi::c_void);
pub const ERROR_INVALID_HANDLE: u32 = 0x6;

impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        matches!(self, &INVALID_HANDLE_VALUE)
    }
}
