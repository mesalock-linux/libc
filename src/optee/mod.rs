//! OP-TEE OS C types definition

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

#[link(name = "utils")]
extern {}

extern {
    // ctypes.h
    pub fn isalpha(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;

    // malloc.h
    pub fn free(p: *mut c_void);
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;

    // stdlib.h
    pub fn abort() -> !;
    pub fn rand() -> c_int;
    pub fn abs(i: c_int) -> c_int;

    // string.h
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void,
                  n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void,
                   n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strndup(cs: *const c_char, maxlen: size_t) -> *mut c_char;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dst: *mut c_char, src: *const c_char,
                   n: size_t) -> *mut c_char;
}

cfg_if! {
    if #[cfg(core_cvoid)] {
        pub use core::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        #[allow(missing_copy_implementations)]
        #[allow(missing_debug_implementations)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}

cfg_if! {
    if #[cfg(target_arch = "arm")] {
        mod arm;
        pub use self::arm::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else {
        // Only tested on ARM so far. Other platforms might have different
        // definitions for types and constants.
        pub use target_arch_not_implemented;
    }
}
