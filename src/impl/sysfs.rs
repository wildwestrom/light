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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn light_file_write_uint64(filename: *const libc::c_char, val: uint64_t) -> bool;
    fn light_file_read_uint64(filename: *const libc::c_char, val: *mut uint64_t) -> bool;
    static mut light_loglevel: light_loglevel_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub struct _impl_sysfs_data_t {
    pub brightness: [libc::c_char; 255],
    pub max_brightness: [libc::c_char; 255],
}
pub type impl_sysfs_data_t = _impl_sysfs_data_t;
pub type DIR = __dirstream;
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
pub const LIGHT_ERROR_LEVEL: light_loglevel_t = 1;
pub type light_loglevel_t = libc::c_uint;
pub const LIGHT_NOTE_LEVEL: light_loglevel_t = 3;
pub const LIGHT_WARN_LEVEL: light_loglevel_t = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
unsafe extern "C" fn _impl_sysfs_init_leds(mut enumerator: *mut light_device_enumerator_t) -> bool {
    let mut leds_device: *mut light_device_t = light_create_device(
        enumerator,
        b"leds\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
    );
    let mut leds_dir: *mut DIR = 0 as *mut DIR;
    let mut curr_entry: *mut dirent = 0 as *mut dirent;
    leds_dir = opendir(b"/sys/class/leds\0" as *const u8 as *const libc::c_char);
    if leds_dir.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            eprintln!("Error: failed to open leds controller directory for reading");
        }
        return 0 as libc::c_int != 0;
    }
    loop {
        curr_entry = readdir(leds_dir);
        if curr_entry.is_null() {
            break;
        }
        if (*curr_entry).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32 {
            continue;
        }
        let mut dev_data: *mut impl_sysfs_data_t =
            malloc(::std::mem::size_of::<impl_sysfs_data_t>() as libc::c_ulong)
                as *mut impl_sysfs_data_t;
        snprintf(
            ((*dev_data).brightness).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"/sys/class/leds/%s/brightness\0" as *const u8 as *const libc::c_char,
            ((*curr_entry).d_name).as_mut_ptr(),
        );
        snprintf(
            ((*dev_data).max_brightness).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"/sys/class/leds/%s/max_brightness\0" as *const u8 as *const libc::c_char,
            ((*curr_entry).d_name).as_mut_ptr(),
        );
        light_create_device_target(
            leds_device,
            ((*curr_entry).d_name).as_mut_ptr(),
            Some(
                impl_sysfs_set
                    as unsafe extern "C" fn(*mut light_device_target_t, uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_get
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_getmax
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_command
                    as unsafe extern "C" fn(
                        *mut light_device_target_t,
                        *const libc::c_char,
                    ) -> bool,
            ),
            dev_data as *mut libc::c_void,
        );
    }
    closedir(leds_dir);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _impl_sysfs_init_backlight(
    mut enumerator: *mut light_device_enumerator_t,
) -> bool {
    let mut backlight_device: *mut light_device_t = light_create_device(
        enumerator,
        b"backlight\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
    );
    let mut backlight_dir: *mut DIR = 0 as *mut DIR;
    let mut curr_entry: *mut dirent = 0 as *mut dirent;
    let mut best_controller: [libc::c_char; 255] = [0; 255];
    let mut best_value: uint64_t = 0 as libc::c_int as uint64_t;
    backlight_dir = opendir(b"/sys/class/backlight\0" as *const u8 as *const libc::c_char);
    if backlight_dir.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            eprintln!("Error: failed to open backlight controller directory for reading");
        }
        return 0 as libc::c_int != 0;
    }
    loop {
        curr_entry = readdir(backlight_dir);
        if curr_entry.is_null() {
            break;
        }
        if (*curr_entry).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32 {
            continue;
        }
        let mut dev_data: *mut impl_sysfs_data_t =
            malloc(::std::mem::size_of::<impl_sysfs_data_t>() as libc::c_ulong)
                as *mut impl_sysfs_data_t;
        snprintf(
            ((*dev_data).brightness).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"/sys/class/backlight/%s/brightness\0" as *const u8 as *const libc::c_char,
            ((*curr_entry).d_name).as_mut_ptr(),
        );
        snprintf(
            ((*dev_data).max_brightness).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"/sys/class/backlight/%s/max_brightness\0" as *const u8 as *const libc::c_char,
            ((*curr_entry).d_name).as_mut_ptr(),
        );
        light_create_device_target(
            backlight_device,
            ((*curr_entry).d_name).as_mut_ptr(),
            Some(
                impl_sysfs_set
                    as unsafe extern "C" fn(*mut light_device_target_t, uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_get
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_getmax
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_command
                    as unsafe extern "C" fn(
                        *mut light_device_target_t,
                        *const libc::c_char,
                    ) -> bool,
            ),
            dev_data as *mut libc::c_void,
        );
        let mut curr_value: uint64_t = 0 as libc::c_int as uint64_t;
        if light_file_read_uint64(((*dev_data).max_brightness).as_mut_ptr(), &mut curr_value) {
            if curr_value > best_value {
                best_value = curr_value;
                snprintf(
                    best_controller.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ((*curr_entry).d_name).as_mut_ptr(),
                );
            }
        }
    }
    closedir(backlight_dir);
    if best_value > 0 as libc::c_int as libc::c_ulong {
        let mut dev_data_0: *mut impl_sysfs_data_t =
            malloc(::std::mem::size_of::<impl_sysfs_data_t>() as libc::c_ulong)
                as *mut impl_sysfs_data_t;
        snprintf(
            ((*dev_data_0).brightness).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"/sys/class/backlight/%s/brightness\0" as *const u8 as *const libc::c_char,
            best_controller.as_mut_ptr(),
        );
        snprintf(
            ((*dev_data_0).max_brightness).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"/sys/class/backlight/%s/max_brightness\0" as *const u8 as *const libc::c_char,
            best_controller.as_mut_ptr(),
        );
        light_create_device_target(
            backlight_device,
            b"auto\0" as *const u8 as *const libc::c_char,
            Some(
                impl_sysfs_set
                    as unsafe extern "C" fn(*mut light_device_target_t, uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_get
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_getmax
                    as unsafe extern "C" fn(*mut light_device_target_t, *mut uint64_t) -> bool,
            ),
            Some(
                impl_sysfs_command
                    as unsafe extern "C" fn(
                        *mut light_device_target_t,
                        *const libc::c_char,
                    ) -> bool,
            ),
            dev_data_0 as *mut libc::c_void,
        );
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_sysfs_init(mut enumerator: *mut light_device_enumerator_t) -> bool {
    _impl_sysfs_init_backlight(enumerator);
    _impl_sysfs_init_leds(enumerator);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_sysfs_free(mut enumerator: *mut light_device_enumerator_t) -> bool {
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_sysfs_set(
    mut target: *mut light_device_target_t,
    mut in_value: uint64_t,
) -> bool {
    let mut data: *mut impl_sysfs_data_t = (*target).device_target_data as *mut impl_sysfs_data_t;
    if !light_file_write_uint64(((*data).brightness).as_mut_ptr(), in_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            eprintln!("Error: failed to write to sysfs device");
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_sysfs_get(
    mut target: *mut light_device_target_t,
    mut out_value: *mut uint64_t,
) -> bool {
    let mut data: *mut impl_sysfs_data_t = (*target).device_target_data as *mut impl_sysfs_data_t;
    if !light_file_read_uint64(((*data).brightness).as_mut_ptr(), out_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            eprintln!("Error: failed to read from sysfs device");
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_sysfs_getmax(
    mut target: *mut light_device_target_t,
    mut out_value: *mut uint64_t,
) -> bool {
    let mut data: *mut impl_sysfs_data_t = (*target).device_target_data as *mut impl_sysfs_data_t;
    if !light_file_read_uint64(((*data).max_brightness).as_mut_ptr(), out_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            eprintln!("Error: failed to read from sysfs device");
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn impl_sysfs_command(
    mut target: *mut light_device_target_t,
    mut command_string: *const libc::c_char,
) -> bool {
    return 1 as libc::c_int != 0;
}
