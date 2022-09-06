use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn light_create_device(
        enumerator: *mut light_device_enumerator_t,
        name: *const libc::c_char,
        device_data: *mut libc::c_void,
    ) -> *mut light_device_t;
    fn light_create_device_target(
        device: *mut light_device_t,
        name: *const libc::c_char,
        setfunc: LFUNCVALSET,
        getfunc: LFUNCVALGET,
        getmaxfunc: LFUNCMAXVALGET,
        cmdfunc: LFUNCCUSTOMCMD,
        target_data: *mut libc::c_void,
    ) -> *mut light_device_target_t;
    static mut stdout: *mut FILE;
    static mut light_loglevel: light_loglevel_t;
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
pub type FILE = _IO_FILE;
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
pub const LIGHT_NOTE_LEVEL: light_loglevel_t = 3;
pub type light_loglevel_t = libc::c_uint;
pub const LIGHT_WARN_LEVEL: light_loglevel_t = 2;
pub const LIGHT_ERROR_LEVEL: light_loglevel_t = 1;
#[no_mangle]
pub unsafe extern "C" fn impl_util_init(mut enumerator: *mut light_device_enumerator_t) -> bool {
    let mut util_device: *mut light_device_t = light_create_device(
        enumerator,
        b"test\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
    );
    light_create_device_target(
        util_device,
        b"dryrun\0" as *const u8 as *const libc::c_char,
        Some(
            impl_util_dryrun_set
                as unsafe extern "C" fn(*mut light_device_target_t, uint64_t) -> bool,
        ),
        Some(
            impl_util_dryrun_get
                as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
        ),
        Some(
            impl_util_dryrun_getmax
                as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
        ),
        Some(
            impl_util_dryrun_command
                as unsafe extern "C" fn(*mut light_device_target_t, *const libc::c_char) -> bool,
        ),
        0 as *mut libc::c_void,
    );
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_util_free(mut enumerator: *mut light_device_enumerator_t) -> bool {
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_util_dryrun_set(
    mut target: *mut light_device_target_t,
    mut in_value: uint64_t,
) -> bool {
    if light_loglevel as libc::c_uint >= LIGHT_NOTE_LEVEL as libc::c_int as libc::c_uint {
        println!(
            "Notice: impl_util_dryrun_set: writing brightness {} to utility target {:?}",
            in_value,
            ((*target).name).as_mut_ptr(),
        );
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_util_dryrun_get(
    mut target: *mut light_device_target_t,
    mut out_value: *mut uint64_t,
) -> bool {
    if light_loglevel as libc::c_uint >= LIGHT_NOTE_LEVEL as libc::c_int as libc::c_uint {
        println!(
            "Notice: impl_util_dryrun_get: reading brightness (0) from utility target {:?}",
            ((*target).name).as_mut_ptr(),
        );
    }
    *out_value = 0 as libc::c_int as uint64_t;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_util_dryrun_getmax(
    mut target: *mut light_device_target_t,
    mut out_value: *mut uint64_t,
) -> bool {
    if light_loglevel as libc::c_uint >= LIGHT_NOTE_LEVEL as libc::c_int as libc::c_uint {
        println!(
            "Notice: impl_util_dryrun_getmax: reading max. brightness (255) from utility target {:?}",
            ((*target).name).as_mut_ptr(),
        );
    }
    *out_value = 255 as libc::c_int as uint64_t;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_util_dryrun_command(
    mut target: *mut light_device_target_t,
    mut command_string: *const libc::c_char,
) -> bool {
    if light_loglevel as libc::c_uint >= LIGHT_NOTE_LEVEL as libc::c_int as libc::c_uint {
        println!(
            "Notice: impl_util_dryrun_command: running custom command on utility target {:?}: \"{:?}\"",
            ((*target).name).as_mut_ptr(),
            command_string,
        );
    }
    return 1 as libc::c_int != 0;
}
