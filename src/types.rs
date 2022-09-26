use thiserror::Error;

#[cfg(feature = "nro")]
#[repr(C)]
pub struct CurlerString {
    pub raw: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

#[cfg(not(feature = "nro"))]
#[repr(C)]
pub struct CurlerString {
    raw: *mut u8,
    len: usize,
    capacity: usize,
}

impl From<CurlerString> for String {
    fn from(other: CurlerString) -> Self {
        unsafe {
            String::from_raw_parts(other.raw, other.len, other.capacity)
        }
    }
}

impl Drop for CurlerString {
    fn drop(&mut self) {
        unsafe {
            drop(String::from_raw_parts(self.raw, self.len, self.capacity))
        }
    }
}

#[cfg(feature = "nro")]
#[export_name = "drop_curler_string"]
pub extern "C" fn drop_curler_string(string: &mut CurlerString) {
    unsafe {
        drop(String::from_raw_parts(string.raw, string.len, string.capacity));
    }
    string.raw = std::ptr::null_mut();
    string.len = 0;
    string.capacity = 0;
}

impl std::fmt::Display for CurlerString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.raw, self.len));
            f.write_str(str)
        }
    }
}

impl std::fmt::Debug for CurlerString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.raw.is_null() {
            f.write_str("<null>")
        } else {
            <Self as std::fmt::Display>::fmt(self, f)
        }
    }
}

#[repr(C)]
#[derive(Error, Debug)]
pub enum HttpCurlError {
    #[error("No error")]
    Ok,

    #[error("cURL is unavailable")]
    CurlUnavailable,

    #[error("The underlying cURL handle is invalid")]
    InvalidHandle,

    #[error("IO Error: {0}")]
    IO(CurlerString),

    #[error("Curl Error: {0:#x}")]
    Curl(i32),
}

#[cfg(not(feature = "nro"))]
#[repr(transparent)]
pub struct Curler(pub(crate) *mut ());