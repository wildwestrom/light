#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn light_initialize(argc: libc::c_int, argv: *mut *mut libc::c_char) -> *mut light_context_t;
    fn light_execute(_: *mut light_context_t) -> bool;
    fn light_free(_: *mut light_context_t);
    static mut stderr: *mut FILE;
    static mut light_loglevel: light_loglevel_t;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _light_device_target_t {
    pub name: [libc::c_char; 256],
    pub set_value: LFUNCVALSET,
    pub get_value: LFUNCVALGET,
    pub get_max_value: LFUNCMAXVALGET,
    pub custom_command: LFUNCCUSTOMCMD,
    pub device_target_data: *mut libc::c_void,
    pub device: *mut light_device_t,
}
pub type light_device_t = _light_device_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _light_device_t {
    pub name: [libc::c_char; 256],
    pub targets: *mut *mut light_device_target_t,
    pub num_targets: uint64_t,
    pub device_data: *mut libc::c_void,
    pub enumerator: *mut light_device_enumerator_t,
}
pub type light_device_enumerator_t = _light_device_enumerator_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _light_device_enumerator_t {
    pub name: [libc::c_char; 256],
    pub init: LFUNCENUMINIT,
    pub free: LFUNCENUMFREE,
    pub devices: *mut *mut light_device_t,
    pub num_devices: uint64_t,
}
pub type LFUNCENUMFREE = Option<unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool>;
pub type LFUNCENUMINIT = Option<unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool>;
pub type light_device_target_t = _light_device_target_t;
pub type LFUNCCUSTOMCMD =
    Option<unsafe extern "C" fn(*mut light_device_target_t, *const libc::c_char) -> bool>;
pub type LFUNCMAXVALGET =
    Option<unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool>;
pub type LFUNCVALGET =
    Option<unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool>;
pub type LFUNCVALSET = Option<unsafe extern "C" fn(*mut light_device_target_t, uint64_t) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _light_context_t {
    pub run_params: C2RustUnnamed_0,
    pub sys_params: C2RustUnnamed,
    pub enumerators: *mut *mut light_device_enumerator_t,
    pub num_enumerators: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub conf_dir: [libc::c_char; 255],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub command: LFUNCCOMMAND,
    pub value: uint64_t,
    pub float_value: libc::c_float,
    pub raw_mode: bool,
    pub device_target: *mut light_device_target_t,
}
pub type LFUNCCOMMAND = Option<unsafe extern "C" fn(*mut light_context_t) -> bool>;
pub type light_context_t = _light_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type light_loglevel_t = libc::c_uint;
pub const LIGHT_NOTE_LEVEL: light_loglevel_t = 3;
pub const LIGHT_WARN_LEVEL: light_loglevel_t = 2;
pub const LIGHT_ERROR_LEVEL: light_loglevel_t = 1;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut light_ctx: *mut light_context_t = light_initialize(argc, argv);
    if light_ctx.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: Initialization failed\n\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int,
            );
        }
        return 2 as libc::c_int;
    }
    if !light_execute(light_ctx) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: Execution failed\n\0" as *const u8 as *const libc::c_char,
                b"main.c\0" as *const u8 as *const libc::c_char,
                20 as libc::c_int,
            );
        }
        light_free(light_ctx);
        return 1 as libc::c_int;
    }
    light_free(light_ctx);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
