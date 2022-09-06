use ::libc;
extern "C" {
    pub type __dirstream;
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
    static mut stderr: *mut FILE;
    static mut light_loglevel: light_loglevel_t;
    fn light_file_read_uint64(filename: *const libc::c_char, val: *mut uint64_t) -> bool;
    fn light_file_write_uint64(filename: *const libc::c_char, val: uint64_t) -> bool;
    fn light_file_exists(filename: *const libc::c_char) -> bool;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type __uint64_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
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
pub struct _impl_razer_data_t {
    pub brightness: [libc::c_char; 255],
    pub max_brightness: uint64_t,
}
pub type impl_razer_data_t = _impl_razer_data_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
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
pub const LIGHT_WARN_LEVEL: light_loglevel_t = 2;
pub type light_loglevel_t = libc::c_uint;
pub const LIGHT_NOTE_LEVEL: light_loglevel_t = 3;
pub const LIGHT_ERROR_LEVEL: light_loglevel_t = 1;
unsafe extern "C" fn _impl_razer_add_target(
    mut device: *mut light_device_t,
    mut name: *const libc::c_char,
    mut filename: *const libc::c_char,
    mut max_brightness: uint64_t,
) {
    let mut target_data: *mut impl_razer_data_t =
        malloc(::std::mem::size_of::<impl_razer_data_t>() as libc::c_ulong)
            as *mut impl_razer_data_t;
    snprintf(
        ((*target_data).brightness).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
        b"/sys/bus/hid/drivers/razerkbd/%s/%s\0" as *const u8 as *const libc::c_char,
        ((*device).name).as_mut_ptr(),
        filename,
    );
    (*target_data).max_brightness = max_brightness;
    if light_file_exists(((*target_data).brightness).as_mut_ptr()) {
        light_create_device_target(
            device,
            name,
            Some(
                impl_razer_set
                    as unsafe extern "C" fn(*mut light_device_target_t, uint64_t) -> bool,
            ),
            Some(
                impl_razer_get
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_razer_getmax
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_razer_command
                    as unsafe extern "C" fn(
                        *mut light_device_target_t,
                        *const libc::c_char,
                    ) -> bool,
            ),
            target_data as *mut libc::c_void,
        );
    } else {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            eprintln!(
                "Warning: razer: couldn't add target {:?} to device {:?}, the file {:?} doesn't exist",
                name,
                ((*device).name).as_mut_ptr(),
                filename,
            );
        }
        free(target_data as *mut libc::c_void);
    };
}
unsafe extern "C" fn _impl_razer_add_device(
    mut enumerator: *mut light_device_enumerator_t,
    mut device_id: *const libc::c_char,
) {
    let mut new_device: *mut light_device_t =
        light_create_device(enumerator, device_id, 0 as *mut libc::c_void);
    _impl_razer_add_target(
        new_device,
        b"backlight\0" as *const u8 as *const libc::c_char,
        b"matrix_brightness\0" as *const u8 as *const libc::c_char,
        255 as libc::c_int as uint64_t,
    );
    _impl_razer_add_target(
        new_device,
        b"game_led\0" as *const u8 as *const libc::c_char,
        b"game_led_state\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
    _impl_razer_add_target(
        new_device,
        b"macro_led\0" as *const u8 as *const libc::c_char,
        b"macro_led_state\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
    _impl_razer_add_target(
        new_device,
        b"logo_led\0" as *const u8 as *const libc::c_char,
        b"logo_led_state\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
    _impl_razer_add_target(
        new_device,
        b"profile_led_r\0" as *const u8 as *const libc::c_char,
        b"profile_led_red\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
    _impl_razer_add_target(
        new_device,
        b"profile_led_g\0" as *const u8 as *const libc::c_char,
        b"profile_led_green\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
    _impl_razer_add_target(
        new_device,
        b"profile_led_b\0" as *const u8 as *const libc::c_char,
        b"profile_led_blue\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn impl_razer_init(mut enumerator: *mut light_device_enumerator_t) -> bool {
    let mut razer_dir: *mut DIR = 0 as *mut DIR;
    let mut curr_entry: *mut dirent = 0 as *mut dirent;
    razer_dir = opendir(b"/sys/bus/hid/drivers/razerkbd/\0" as *const u8 as *const libc::c_char);
    if razer_dir.is_null() {
        return 1 as libc::c_int != 0;
    }
    loop {
        curr_entry = readdir(razer_dir);
        if curr_entry.is_null() {
            break;
        }
        if (*curr_entry).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32 {
            continue;
        }
        _impl_razer_add_device(enumerator, ((*curr_entry).d_name).as_mut_ptr());
    }
    closedir(razer_dir);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_razer_free(mut enumerator: *mut light_device_enumerator_t) -> bool {
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_razer_set(
    mut target: *mut light_device_target_t,
    mut in_value: uint64_t,
) -> bool {
    let mut data: *mut impl_razer_data_t = (*target).device_target_data as *mut impl_razer_data_t;
    if !light_file_write_uint64(((*data).brightness).as_mut_ptr(), in_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            eprintln!("Error: failed to write to razer device",);
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_razer_get(
    mut target: *mut light_device_target_t,
    mut out_value: *mut uint64_t,
) -> bool {
    let mut data: *mut impl_razer_data_t = (*target).device_target_data as *mut impl_razer_data_t;
    if !light_file_read_uint64(((*data).brightness).as_mut_ptr(), out_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            eprintln!("Error: failed to read from razer device",);
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_razer_getmax(
    mut target: *mut light_device_target_t,
    mut out_value: *mut uint64_t,
) -> bool {
    let mut data: *mut impl_razer_data_t = (*target).device_target_data as *mut impl_razer_data_t;
    *out_value = (*data).max_brightness;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_razer_command(
    mut target: *mut light_device_target_t,
    mut command_string: *const libc::c_char,
) -> bool {
    return 1 as libc::c_int != 0;
}
