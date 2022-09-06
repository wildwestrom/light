use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    static mut light_loglevel: light_loglevel_t;
    fn light_file_read_uint64(filename: *const libc::c_char, val: *mut uint64_t) -> bool;
    fn light_mkpath(dir: *mut libc::c_char, mode: mode_t) -> libc::c_int;
    fn light_percent_clamp(percent: libc::c_double) -> libc::c_double;
    fn light_file_write_uint64(filename: *const libc::c_char, val: uint64_t) -> bool;
    fn light_log_clamp_max(max: uint64_t) -> uint64_t;
    fn light_log_clamp_min(min: uint64_t) -> uint64_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn impl_sysfs_init(enumerator: *mut light_device_enumerator_t) -> bool;
    fn impl_sysfs_free(enumerator: *mut light_device_enumerator_t) -> bool;
    fn impl_util_init(enumerator: *mut light_device_enumerator_t) -> bool;
    fn impl_util_free(enumerator: *mut light_device_enumerator_t) -> bool;
    fn impl_razer_init(enumerator: *mut light_device_enumerator_t) -> bool;
    fn impl_razer_free(enumerator: *mut light_device_enumerator_t) -> bool;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getegid() -> __gid_t;
    fn setegid(__gid: __gid_t) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
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
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type light_target_path_t = _light_target_path_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _light_target_path_t {
    pub enumerator: [libc::c_char; 255],
    pub device: [libc::c_char; 255],
    pub target: [libc::c_char; 255],
}
pub type gid_t = __gid_t;
unsafe extern "C" fn _light_add_enumerator_device(
    mut enumerator: *mut light_device_enumerator_t,
    mut new_device: *mut light_device_t,
) {
    let mut new_num_devices: uint64_t =
        ((*enumerator).num_devices).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut new_devices: *mut *mut light_device_t = malloc(
        new_num_devices.wrapping_mul(::std::mem::size_of::<*mut light_device_t>() as libc::c_ulong),
    ) as *mut *mut light_device_t;
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < (*enumerator).num_devices {
        let ref mut fresh0 = *new_devices.offset(i as isize);
        *fresh0 = *((*enumerator).devices).offset(i as isize);
        i = i.wrapping_add(1);
    }
    let ref mut fresh1 = *new_devices.offset((*enumerator).num_devices as isize);
    *fresh1 = new_device;
    if !((*enumerator).devices).is_null() {
        free((*enumerator).devices as *mut libc::c_void);
    }
    let ref mut fresh2 = (*enumerator).devices;
    *fresh2 = new_devices;
    (*enumerator).num_devices = new_num_devices;
}
unsafe extern "C" fn _light_add_device_target(
    mut device: *mut light_device_t,
    mut new_target: *mut light_device_target_t,
) {
    let mut new_num_targets: uint64_t =
        ((*device).num_targets).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut new_targets: *mut *mut light_device_target_t = malloc(
        new_num_targets
            .wrapping_mul(::std::mem::size_of::<*mut light_device_target_t>() as libc::c_ulong),
    ) as *mut *mut light_device_target_t;
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < (*device).num_targets {
        let ref mut fresh3 = *new_targets.offset(i as isize);
        *fresh3 = *((*device).targets).offset(i as isize);
        i = i.wrapping_add(1);
    }
    let ref mut fresh4 = *new_targets.offset((*device).num_targets as isize);
    *fresh4 = new_target;
    if !((*device).targets).is_null() {
        free((*device).targets as *mut libc::c_void);
    }
    let ref mut fresh5 = (*device).targets;
    *fresh5 = new_targets;
    (*device).num_targets = new_num_targets;
}
unsafe extern "C" fn _light_rc_initialize(mut new_ctx: *mut light_context_t) -> bool {
    let mut euid: uid_t = geteuid();
    if euid == 0 as libc::c_int as libc::c_uint {
        snprintf(
            ((*new_ctx).sys_params.conf_dir).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"/etc/light\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut xdg_conf: *mut libc::c_char =
            getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char);
        if !xdg_conf.is_null() {
            snprintf(
                ((*new_ctx).sys_params.conf_dir).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                b"%s/light\0" as *const u8 as *const libc::c_char,
                xdg_conf,
            );
        } else {
            snprintf(
                ((*new_ctx).sys_params.conf_dir).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                b"%s/.config/light\0" as *const u8 as *const libc::c_char,
                getenv(b"HOME\0" as *const u8 as *const libc::c_char),
            );
        }
    }
    let mut rc: int32_t = light_mkpath(
        ((*new_ctx).sys_params.conf_dir).as_mut_ptr(),
        (0o400 as libc::c_int
            | 0o200 as libc::c_int
            | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if rc != 0 && *__errno_location() != 17 as libc::c_int {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: couldn't create configuration directory\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _light_get_target_path(
    mut ctx: *mut light_context_t,
    mut output_path: *mut libc::c_char,
    mut output_size: size_t,
) {
    if _light_rc_initialize(ctx) {
        snprintf(
            output_path,
            output_size,
            b"%s/targets/%s/%s/%s\0" as *const u8 as *const libc::c_char,
            ((*ctx).sys_params.conf_dir).as_mut_ptr(),
            ((*(*(*(*ctx).run_params.device_target).device).enumerator).name).as_mut_ptr(),
            ((*(*(*ctx).run_params.device_target).device).name).as_mut_ptr(),
            ((*(*ctx).run_params.device_target).name).as_mut_ptr(),
        );
    }
}
unsafe extern "C" fn _light_get_target_file(
    mut ctx: *mut light_context_t,
    mut output_path: *mut libc::c_char,
    mut output_size: size_t,
    mut file: *const libc::c_char,
) {
    if _light_rc_initialize(ctx) {
        snprintf(
            output_path,
            output_size,
            b"%s/targets/%s/%s/%s/%s\0" as *const u8 as *const libc::c_char,
            ((*ctx).sys_params.conf_dir).as_mut_ptr(),
            ((*(*(*(*ctx).run_params.device_target).device).enumerator).name).as_mut_ptr(),
            ((*(*(*ctx).run_params.device_target).device).name).as_mut_ptr(),
            ((*(*ctx).run_params.device_target).name).as_mut_ptr(),
            file,
        );
    }
}
unsafe extern "C" fn _light_get_min_cap(mut ctx: *mut light_context_t) -> uint64_t {
    let mut target_path: [libc::c_char; 255] = [0; 255];
    _light_get_target_file(
        ctx,
        target_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
        b"minimum\0" as *const u8 as *const libc::c_char,
    );
    let mut minimum_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !light_file_read_uint64(target_path.as_mut_ptr(), &mut minimum_value) {
        return 0 as libc::c_int as uint64_t;
    }
    return minimum_value;
}
unsafe extern "C" fn _light_find_enumerator(
    mut ctx: *mut light_context_t,
    mut comp: *const libc::c_char,
) -> *mut light_device_enumerator_t {
    let mut e: uint64_t = 0 as libc::c_int as uint64_t;
    while e < (*ctx).num_enumerators {
        if strncmp(
            comp,
            ((**((*ctx).enumerators).offset(e as isize)).name).as_mut_ptr(),
            255 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return *((*ctx).enumerators).offset(e as isize);
        }
        e = e.wrapping_add(1);
    }
    return 0 as *mut light_device_enumerator_t;
}
unsafe extern "C" fn _light_find_device(
    mut en: *mut light_device_enumerator_t,
    mut comp: *const libc::c_char,
) -> *mut light_device_t {
    let mut d: uint64_t = 0 as libc::c_int as uint64_t;
    while d < (*en).num_devices {
        if strncmp(
            comp,
            ((**((*en).devices).offset(d as isize)).name).as_mut_ptr(),
            255 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return *((*en).devices).offset(d as isize);
        }
        d = d.wrapping_add(1);
    }
    return 0 as *mut light_device_t;
}
unsafe extern "C" fn _light_find_target(
    mut dev: *mut light_device_t,
    mut comp: *const libc::c_char,
) -> *mut light_device_target_t {
    let mut t: uint64_t = 0 as libc::c_int as uint64_t;
    while t < (*dev).num_targets {
        if strncmp(
            comp,
            ((**((*dev).targets).offset(t as isize)).name).as_mut_ptr(),
            255 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return *((*dev).targets).offset(t as isize);
        }
        t = t.wrapping_add(1);
    }
    return 0 as *mut light_device_target_t;
}
unsafe extern "C" fn _light_raw_to_percent(
    mut target: *mut light_device_target_t,
    mut inraw: uint64_t,
    mut outpercent: *mut libc::c_double,
) -> bool {
    let mut inraw_d: libc::c_double = inraw as libc::c_double;
    let mut max_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_max_value).expect("non-null function pointer")(target, &mut max_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                187 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut max_value_d: libc::c_double = max_value as libc::c_double;
    let mut percent: libc::c_double = light_percent_clamp(inraw_d / max_value_d * 100.0f64);
    *outpercent = percent;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _light_percent_to_raw(
    mut target: *mut light_device_target_t,
    mut inpercent: libc::c_double,
    mut outraw: *mut uint64_t,
) -> bool {
    let mut max_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_max_value).expect("non-null function pointer")(target, &mut max_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                202 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut max_value_d: libc::c_double = max_value as libc::c_double;
    let mut target_value_d: libc::c_double =
        max_value_d * (light_percent_clamp(inpercent) / 100.0f64);
    let mut target_value: uint64_t =
        if (target_value_d as uint64_t) < 0 as libc::c_int as libc::c_ulong {
            light_log_clamp_min(0 as libc::c_int as uint64_t)
        } else if target_value_d as uint64_t > max_value {
            light_log_clamp_max(max_value)
        } else {
            target_value_d as uint64_t
        };
    *outraw = target_value;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _light_print_usage() {
    printf(
        b"Usage:\n  light [OPTIONS] [VALUE]\n\nCommands:\n  -H, -h      Show this help and exit\n  -V          Show program version and exit\n  -L          List available devices\n  -A          Increase brightness by value\n  -U          Decrease brightness by value\n  -T          Multiply brightness by value (can be a non-whole number, ignores raw mode)\n  -S          Set brightness to value\n  -G          Get brightness\n  -N          Set minimum brightness to value\n  -P          Get minimum brightness\n  -O          Save the current brightness\n  -I          Restore the previously saved brightness\n\nOptions:\n  -r          Interpret input and output values in raw mode (ignored for -T)\n  -s          Specify device target path to use, use -L to list available\n  -v          Specify the verbosity level (default 0)\n                 0: Values only\n                 1: Values, Errors.\n                 2: Values, Errors, Warnings.\n                 3: Values, Errors, Warnings, Notices.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Copyright (C) %s  %s\n\0" as *const u8 as *const libc::c_char,
        b"2012 - 2018\0" as *const u8 as *const libc::c_char,
        b"Fredrik Haikarainen\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"This is free software, see the source for copying conditions.  There is NO\nwarranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE\n\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn _light_set_context_command(
    mut ctx: *mut light_context_t,
    mut new_cmd: LFUNCCOMMAND,
) -> bool {
    if ((*ctx).run_params.command).is_some() {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: a command was already set. ignoring.\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                256 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh6 = (*ctx).run_params.command;
    *fresh6 = new_cmd;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _light_parse_arguments(
    mut ctx: *mut light_context_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> bool {
    let mut curr_arg: int32_t = -(1 as libc::c_int);
    let mut log_level: int32_t = 0 as libc::c_int;
    let mut ctrl_name: [libc::c_char; 255] = [0; 255];
    let mut need_value: bool = 0 as libc::c_int != 0;
    let mut need_float_value: bool = 0 as libc::c_int != 0;
    let mut need_target: bool = 1 as libc::c_int != 0;
    let mut specified_target: bool = 0 as libc::c_int != 0;
    snprintf(
        ctrl_name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"sysfs/backlight/auto\0" as *const u8 as *const libc::c_char,
    );
    loop {
        curr_arg = getopt(
            argc,
            argv,
            b"HhVGSLMNPAUTOIv:s:r\0" as *const u8 as *const libc::c_char,
        );
        if !(curr_arg != -(1 as libc::c_int)) {
            break;
        }
        match curr_arg {
            118 => {
                if sscanf(
                    optarg,
                    b"%i\0" as *const u8 as *const libc::c_char,
                    &mut log_level as *mut int32_t,
                ) != 1 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"-v argument is not an integer.\n\n\0" as *const u8 as *const libc::c_char,
                    );
                    _light_print_usage();
                    return 0 as libc::c_int != 0;
                }
                if log_level < 0 as libc::c_int || log_level > 3 as libc::c_int {
                    fprintf(
                        stderr,
                        b"-v argument must be between 0 and 3.\n\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    _light_print_usage();
                    return 0 as libc::c_int != 0;
                }
                light_loglevel = log_level as light_loglevel_t;
            }
            115 => {
                snprintf(
                    ctrl_name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
                specified_target = 1 as libc::c_int != 0;
            }
            114 => {
                (*ctx).run_params.raw_mode = 1 as libc::c_int != 0;
            }
            72 | 104 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_print_help as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
            }
            86 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_print_version
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
            }
            71 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_get_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
            }
            83 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_set_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_value = 1 as libc::c_int != 0;
                need_target = 1 as libc::c_int != 0;
            }
            76 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_list_devices
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
            }
            77 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_get_max_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
            }
            78 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_set_min_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
                need_value = 1 as libc::c_int != 0;
            }
            80 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_get_min_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
            }
            65 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_add_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
                need_value = 1 as libc::c_int != 0;
            }
            85 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_sub_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
                need_value = 1 as libc::c_int != 0;
            }
            84 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_mul_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
                need_float_value = 1 as libc::c_int != 0;
            }
            79 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_save_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
            }
            73 => {
                _light_set_context_command(
                    ctx,
                    Some(
                        light_cmd_restore_brightness
                            as unsafe extern "C" fn(*mut light_context_t) -> bool,
                    ),
                );
                need_target = 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if ((*ctx).run_params.command).is_none() {
        _light_set_context_command(
            ctx,
            Some(light_cmd_get_brightness as unsafe extern "C" fn(*mut light_context_t) -> bool),
        );
    }
    if need_target {
        let mut curr_target: *mut light_device_target_t =
            light_find_device_target(ctx, ctrl_name.as_mut_ptr());
        if curr_target.is_null() {
            if specified_target {
                fprintf(
                    stderr,
                    b"We couldn't find the specified device target at the path \"%s\". Use -L to find one.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    ctrl_name.as_mut_ptr(),
                );
                return 0 as libc::c_int != 0;
            } else {
                fprintf(
                    stderr,
                    b"No backlight controller was found, so we could not decide an automatic target. The current command will have no effect. Please use -L to find a target and then specify it with -s.\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
                curr_target = light_find_device_target(
                    ctx,
                    b"util/test/dryrun\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        let ref mut fresh7 = (*ctx).run_params.device_target;
        *fresh7 = curr_target;
    }
    if need_value as libc::c_int != 0 || need_float_value as libc::c_int != 0 {
        if argc - optind != 1 as libc::c_int {
            fprintf(
                stderr,
                b"please specify a <value> for this command.\n\n\0" as *const u8
                    as *const libc::c_char,
            );
            _light_print_usage();
            return 0 as libc::c_int != 0;
        }
    }
    if need_value {
        if (*ctx).run_params.raw_mode {
            if sscanf(
                *argv.offset(optind as isize),
                b"%lu\0" as *const u8 as *const libc::c_char,
                &mut (*ctx).run_params.value as *mut uint64_t,
            ) != 1 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"<value> is not an integer.\n\n\0" as *const u8 as *const libc::c_char,
                );
                _light_print_usage();
                return 0 as libc::c_int != 0;
            }
        } else {
            let mut percent_value: libc::c_double = 0.0f64;
            if sscanf(
                *argv.offset(optind as isize),
                b"%lf\0" as *const u8 as *const libc::c_char,
                &mut percent_value as *mut libc::c_double,
            ) != 1 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"<value> is not a decimal.\n\n\0" as *const u8 as *const libc::c_char,
                );
                _light_print_usage();
                return 0 as libc::c_int != 0;
            }
            percent_value = light_percent_clamp(percent_value);
            let mut raw_value: uint64_t = 0 as libc::c_int as uint64_t;
            if !_light_percent_to_raw(
                (*ctx).run_params.device_target,
                percent_value,
                &mut raw_value,
            ) {
                if light_loglevel as libc::c_uint
                    >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint
                {
                    fprintf(
                        stderr,
                        b"%s:%d: Error: failed to convert from percent to raw for device target\n\0"
                            as *const u8 as *const libc::c_char,
                        b"light.c\0" as *const u8 as *const libc::c_char,
                        427 as libc::c_int,
                    );
                }
                return 0 as libc::c_int != 0;
            }
            (*ctx).run_params.value = raw_value;
        }
    }
    if need_float_value {
        if sscanf(
            *argv.offset(optind as isize),
            b"%f\0" as *const u8 as *const libc::c_char,
            &mut (*ctx).run_params.float_value as *mut libc::c_float,
        ) != 1 as libc::c_int
        {
            fprintf(
                stderr,
                b"<value> is not a float.\n\n\0" as *const u8 as *const libc::c_char,
            );
            _light_print_usage();
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_initialize(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut light_context_t {
    let mut new_ctx: *mut light_context_t =
        malloc(::std::mem::size_of::<light_context_t>() as libc::c_ulong) as *mut light_context_t;
    let ref mut fresh8 = (*new_ctx).enumerators;
    *fresh8 = 0 as *mut *mut light_device_enumerator_t;
    (*new_ctx).num_enumerators = 0 as libc::c_int as uint64_t;
    let ref mut fresh9 = (*new_ctx).run_params.command;
    *fresh9 = None;
    let ref mut fresh10 = (*new_ctx).run_params.device_target;
    *fresh10 = 0 as *mut light_device_target_t;
    (*new_ctx).run_params.value = 0 as libc::c_int as uint64_t;
    (*new_ctx).run_params.raw_mode = 0 as libc::c_int != 0;
    let mut uid: uid_t = getuid();
    let mut euid: uid_t = geteuid();
    let mut egid: gid_t = getegid();
    if uid != euid
        && euid == 0 as libc::c_int as libc::c_uint
        && egid != 0 as libc::c_int as libc::c_uint
    {
        if setegid(euid) < 0 as libc::c_int {
            if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"%s:%d: Error: could not change egid from %u to %u (uid: %u, euid: %u)\n\0"
                        as *const u8 as *const libc::c_char,
                    b"light.c\0" as *const u8 as *const libc::c_char,
                    477 as libc::c_int,
                    egid,
                    euid,
                    uid,
                    euid,
                );
            }
            return 0 as *mut light_context_t;
        }
    }
    light_create_enumerator(
        new_ctx,
        b"sysfs\0" as *const u8 as *const libc::c_char,
        Some(impl_sysfs_init as unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool),
        Some(impl_sysfs_free as unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool),
    );
    light_create_enumerator(
        new_ctx,
        b"util\0" as *const u8 as *const libc::c_char,
        Some(impl_util_init as unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool),
        Some(impl_util_free as unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool),
    );
    light_create_enumerator(
        new_ctx,
        b"razer\0" as *const u8 as *const libc::c_char,
        Some(impl_razer_init as unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool),
        Some(impl_razer_free as unsafe extern "C" fn(*mut light_device_enumerator_t) -> bool),
    );
    if !light_init_enumerators(new_ctx) {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: failed to initialize all enumerators\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                494 as libc::c_int,
            );
        }
    }
    if !_light_parse_arguments(new_ctx, argc, argv) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to parse arguments\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                500 as libc::c_int,
            );
        }
        return 0 as *mut light_context_t;
    }
    return new_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn light_execute(mut ctx: *mut light_context_t) -> bool {
    if ((*ctx).run_params.command).is_none() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: run parameters command was null, can't execute\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                511 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return ((*ctx).run_params.command).expect("non-null function pointer")(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn light_free(mut ctx: *mut light_context_t) {
    if !light_free_enumerators(ctx) {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: failed to free all enumerators\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                522 as libc::c_int,
            );
        }
    }
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn light_create_enumerator(
    mut ctx: *mut light_context_t,
    mut name: *const libc::c_char,
    mut init_func: LFUNCENUMINIT,
    mut free_func: LFUNCENUMFREE,
) -> *mut light_device_enumerator_t {
    let mut new_num_enumerators: uint64_t =
        ((*ctx).num_enumerators).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut new_enumerators: *mut *mut light_device_enumerator_t = malloc(
        new_num_enumerators
            .wrapping_mul(::std::mem::size_of::<*mut light_device_enumerator_t>() as libc::c_ulong),
    )
        as *mut *mut light_device_enumerator_t;
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < (*ctx).num_enumerators {
        let ref mut fresh11 = *new_enumerators.offset(i as isize);
        *fresh11 = *((*ctx).enumerators).offset(i as isize);
        i = i.wrapping_add(1);
    }
    let ref mut fresh12 = *new_enumerators.offset((*ctx).num_enumerators as isize);
    *fresh12 = malloc(::std::mem::size_of::<light_device_enumerator_t>() as libc::c_ulong)
        as *mut light_device_enumerator_t;
    let mut returner: *mut light_device_enumerator_t =
        *new_enumerators.offset((*ctx).num_enumerators as isize);
    let ref mut fresh13 = (*returner).devices;
    *fresh13 = 0 as *mut *mut light_device_t;
    (*returner).num_devices = 0 as libc::c_int as uint64_t;
    let ref mut fresh14 = (*returner).init;
    *fresh14 = init_func;
    let ref mut fresh15 = (*returner).free;
    *fresh15 = free_func;
    snprintf(
        ((*returner).name).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    if !((*ctx).enumerators).is_null() {
        free((*ctx).enumerators as *mut libc::c_void);
    }
    let ref mut fresh16 = (*ctx).enumerators;
    *fresh16 = new_enumerators;
    (*ctx).num_enumerators = new_num_enumerators;
    return returner;
}
#[no_mangle]
pub unsafe extern "C" fn light_init_enumerators(mut ctx: *mut light_context_t) -> bool {
    let mut success: bool = 1 as libc::c_int != 0;
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < (*ctx).num_enumerators {
        let mut curr_enumerator: *mut light_device_enumerator_t =
            *((*ctx).enumerators).offset(i as isize);
        if !((*curr_enumerator).init).expect("non-null function pointer")(curr_enumerator) {
            success = 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn light_free_enumerators(mut ctx: *mut light_context_t) -> bool {
    let mut success: bool = 1 as libc::c_int != 0;
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < (*ctx).num_enumerators {
        let mut curr_enumerator: *mut light_device_enumerator_t =
            *((*ctx).enumerators).offset(i as isize);
        if !((*curr_enumerator).free).expect("non-null function pointer")(curr_enumerator) {
            success = 0 as libc::c_int != 0;
        }
        if !((*curr_enumerator).devices).is_null() {
            let mut d: uint64_t = 0 as libc::c_int as uint64_t;
            while d < (*curr_enumerator).num_devices {
                light_delete_device(*((*curr_enumerator).devices).offset(d as isize));
                d = d.wrapping_add(1);
            }
            free((*curr_enumerator).devices as *mut libc::c_void);
            let ref mut fresh17 = (*curr_enumerator).devices;
            *fresh17 = 0 as *mut *mut light_device_t;
        }
        free(curr_enumerator as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free((*ctx).enumerators as *mut libc::c_void);
    let ref mut fresh18 = (*ctx).enumerators;
    *fresh18 = 0 as *mut *mut light_device_enumerator_t;
    (*ctx).num_enumerators = 0 as libc::c_int as uint64_t;
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn light_split_target_path(
    mut in_path: *const libc::c_char,
    mut out_path: *mut light_target_path_t,
) -> bool {
    let mut begin: *const libc::c_char = in_path;
    let mut end: *const libc::c_char = strstr(begin, b"/\0" as *const u8 as *const libc::c_char);
    if end.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: invalid path passed to split_target_path\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                618 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut size: size_t = end.offset_from(begin) as libc::c_long as size_t;
    strncpy(((*out_path).enumerator).as_mut_ptr(), begin, size);
    (*out_path).enumerator[size as usize] = '\0' as i32 as libc::c_char;
    begin = end.offset(1 as libc::c_int as isize);
    end = strstr(begin, b"/\0" as *const u8 as *const libc::c_char);
    if end.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: invalid path passed to split_target_path\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                630 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    size = end.offset_from(begin) as libc::c_long as size_t;
    strncpy(((*out_path).device).as_mut_ptr(), begin, size);
    (*out_path).device[size as usize] = '\0' as i32 as libc::c_char;
    strcpy(
        ((*out_path).target).as_mut_ptr(),
        end.offset(1 as libc::c_int as isize),
    );
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_find_device_target(
    mut ctx: *mut light_context_t,
    mut name: *const libc::c_char,
) -> *mut light_device_target_t {
    let mut new_path: light_target_path_t = light_target_path_t {
        enumerator: [0; 255],
        device: [0; 255],
        target: [0; 255],
    };
    if !light_split_target_path(name, &mut new_path) {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: light_find_device_target needs a path in the format of \"enumerator/device/target\", the following format is not recognized:  \"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                648 as libc::c_int,
                name,
            );
        }
        return 0 as *mut light_device_target_t;
    }
    let mut enumerator: *mut light_device_enumerator_t =
        _light_find_enumerator(ctx, (new_path.enumerator).as_mut_ptr());
    if enumerator.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: no such enumerator, \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                665 as libc::c_int,
                (new_path.enumerator).as_mut_ptr(),
            );
        }
        return 0 as *mut light_device_target_t;
    }
    let mut device: *mut light_device_t =
        _light_find_device(enumerator, (new_path.device).as_mut_ptr());
    if device.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: no such device, \"%s\"\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                672 as libc::c_int,
                (new_path.device).as_mut_ptr(),
            );
        }
        return 0 as *mut light_device_target_t;
    }
    let mut target: *mut light_device_target_t =
        _light_find_target(device, (new_path.target).as_mut_ptr());
    if target.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Warning: no such target, \"%s\"\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                679 as libc::c_int,
                (new_path.target).as_mut_ptr(),
            );
        }
        return 0 as *mut light_device_target_t;
    }
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_print_help(mut ctx: *mut light_context_t) -> bool {
    _light_print_usage();
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_print_version(mut ctx: *mut light_context_t) -> bool {
    printf(
        b"v%s\n\0" as *const u8 as *const libc::c_char,
        b"1.2\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_list_devices(mut ctx: *mut light_context_t) -> bool {
    printf(b"Listing device targets:\n\0" as *const u8 as *const libc::c_char);
    let mut enumerator: uint64_t = 0 as libc::c_int as uint64_t;
    while enumerator < (*ctx).num_enumerators {
        let mut curr_enumerator: *mut light_device_enumerator_t =
            *((*ctx).enumerators).offset(enumerator as isize);
        let mut device: uint64_t = 0 as libc::c_int as uint64_t;
        while device < (*curr_enumerator).num_devices {
            let mut curr_device: *mut light_device_t =
                *((*curr_enumerator).devices).offset(device as isize);
            let mut target: uint64_t = 0 as libc::c_int as uint64_t;
            while target < (*curr_device).num_targets {
                let mut curr_target: *mut light_device_target_t =
                    *((*curr_device).targets).offset(target as isize);
                printf(
                    b"\t%s/%s/%s\n\0" as *const u8 as *const libc::c_char,
                    ((*curr_enumerator).name).as_mut_ptr(),
                    ((*curr_device).name).as_mut_ptr(),
                    ((*curr_target).name).as_mut_ptr(),
                );
                target = target.wrapping_add(1);
            }
            device = device.wrapping_add(1);
        }
        enumerator = enumerator.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_set_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target: *mut light_device_target_t = (*ctx).run_params.device_target;
    if target.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: didn't have a valid target, programmer mistake\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                724 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut mincap: uint64_t = _light_get_min_cap(ctx);
    let mut value: uint64_t = (*ctx).run_params.value;
    if mincap > value {
        value = mincap;
    }
    if !((*target).set_value).expect("non-null function pointer")(target, value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to write to target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                738 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_get_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target: *mut light_device_target_t = (*ctx).run_params.device_target;
    if target.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: didn't have a valid target, programmer mistake\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                750 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_value).expect("non-null function pointer")(target, &mut value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                757 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if (*ctx).run_params.raw_mode {
        printf(b"%lu\n\0" as *const u8 as *const libc::c_char, value);
    } else {
        let mut percent: libc::c_double = 0.0f64;
        if !_light_raw_to_percent(target, value, &mut percent) {
            if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"%s:%d: Error: failed to convert from raw to percent from device target\n\0"
                        as *const u8 as *const libc::c_char,
                    b"light.c\0" as *const u8 as *const libc::c_char,
                    770 as libc::c_int,
                );
            }
            return 0 as libc::c_int != 0;
        }
        printf(b"%.2f\n\0" as *const u8 as *const libc::c_char, percent);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_get_max_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target: *mut light_device_target_t = (*ctx).run_params.device_target;
    if target.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: didn't have a valid target, programmer mistake\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                784 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if !(*ctx).run_params.raw_mode {
        printf(b"100.0\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int != 0;
    }
    let mut max_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_max_value).expect("non-null function pointer")(target, &mut max_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to read from device target\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                797 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    printf(b"%lu\n\0" as *const u8 as *const libc::c_char, max_value);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_set_min_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target_path: [libc::c_char; 255] = [0; 255];
    _light_get_target_path(
        ctx,
        target_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
    );
    let mut rc: int32_t = light_mkpath(
        target_path.as_mut_ptr(),
        (0o400 as libc::c_int
            | 0o200 as libc::c_int
            | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if rc != 0 && *__errno_location() != 17 as libc::c_int {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't create target directory for minimum brightness\n\0"
                    as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                814 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut target_filepath: [libc::c_char; 255] = [0; 255];
    _light_get_target_file(
        ctx,
        target_filepath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
        b"minimum\0" as *const u8 as *const libc::c_char,
    );
    if !light_file_write_uint64(target_filepath.as_mut_ptr(), (*ctx).run_params.value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't write value to minimum file\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                823 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_get_min_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target_path: [libc::c_char; 255] = [0; 255];
    _light_get_target_file(
        ctx,
        target_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
        b"minimum\0" as *const u8 as *const libc::c_char,
    );
    let mut minimum_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !light_file_read_uint64(target_path.as_mut_ptr(), &mut minimum_value) {
        if (*ctx).run_params.raw_mode {
            printf(b"0\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"0.00\n\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int != 0;
    }
    if (*ctx).run_params.raw_mode {
        printf(
            b"%lu\n\0" as *const u8 as *const libc::c_char,
            minimum_value,
        );
    } else {
        let mut minimum_d: libc::c_double = 0.0f64;
        if !_light_raw_to_percent(
            (*ctx).run_params.device_target,
            minimum_value,
            &mut minimum_d,
        ) {
            if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"%s:%d: Error: failed to convert value from raw to percent for device target\n\0"
                        as *const u8 as *const libc::c_char,
                    b"light.c\0" as *const u8 as *const libc::c_char,
                    859 as libc::c_int,
                );
            }
            return 0 as libc::c_int != 0;
        }
        printf(b"%.2f\n\0" as *const u8 as *const libc::c_char, minimum_d);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_add_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target: *mut light_device_target_t = (*ctx).run_params.device_target;
    if target.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: didn't have a valid target, programmer mistake\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                874 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_value).expect("non-null function pointer")(target, &mut value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                881 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut max_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_max_value).expect("non-null function pointer")(target, &mut max_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                888 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    value = (value as libc::c_ulong).wrapping_add((*ctx).run_params.value) as uint64_t as uint64_t;
    let mut mincap: uint64_t = _light_get_min_cap(ctx);
    if mincap > value {
        value = mincap;
    }
    if value > max_value {
        value = max_value;
    }
    if !((*target).set_value).expect("non-null function pointer")(target, value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to write to target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                908 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_sub_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target: *mut light_device_target_t = (*ctx).run_params.device_target;
    if target.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: didn't have a valid target, programmer mistake\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                920 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_value).expect("non-null function pointer")(target, &mut value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                927 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if value > (*ctx).run_params.value {
        value =
            (value as libc::c_ulong).wrapping_sub((*ctx).run_params.value) as uint64_t as uint64_t;
    } else {
        value = 0 as libc::c_int as uint64_t;
    }
    let mut mincap: uint64_t = _light_get_min_cap(ctx);
    if mincap > value {
        value = mincap;
    }
    if !((*target).set_value).expect("non-null function pointer")(target, value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to write to target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                948 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_mul_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target: *mut light_device_target_t = (*ctx).run_params.device_target;
    if target.is_null() {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: didn't have a valid target, programmer mistake\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                960 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_value).expect("non-null function pointer")(target, &mut value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                967 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut max_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*target).get_max_value).expect("non-null function pointer")(target, &mut max_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to read from target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                974 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut old_value: uint64_t = value;
    value = (value as libc::c_float * (*ctx).run_params.float_value) as uint64_t;
    if value == old_value {
        if (*ctx).run_params.float_value > 1 as libc::c_int as libc::c_float {
            value = value.wrapping_add(1);
        }
        if (*ctx).run_params.float_value < 1 as libc::c_int as libc::c_float
            && value > 0 as libc::c_int as libc::c_ulong
        {
            value = value.wrapping_sub(1);
        }
    }
    let mut mincap: uint64_t = _light_get_min_cap(ctx);
    if mincap > value {
        value = mincap;
    }
    if value > max_value {
        value = max_value;
    }
    if !((*target).set_value).expect("non-null function pointer")(target, value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: failed to write to target\n\0" as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                1003 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_save_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target_path: [libc::c_char; 255] = [0; 255];
    _light_get_target_path(
        ctx,
        target_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
    );
    let mut rc: int32_t = light_mkpath(
        target_path.as_mut_ptr(),
        (0o400 as libc::c_int
            | 0o200 as libc::c_int
            | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if rc != 0 && *__errno_location() != 17 as libc::c_int {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't create target directory for save brightness\n\0"
                    as *const u8 as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                1019 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut target_filepath: [libc::c_char; 255] = [0; 255];
    _light_get_target_file(
        ctx,
        target_filepath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
        b"save\0" as *const u8 as *const libc::c_char,
    );
    let mut curr_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !((*(*ctx).run_params.device_target).get_value).expect("non-null function pointer")(
        (*ctx).run_params.device_target,
        &mut curr_value,
    ) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't read from device target\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                1029 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if !light_file_write_uint64(target_filepath.as_mut_ptr(), curr_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't write value to savefile\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                1035 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_cmd_restore_brightness(mut ctx: *mut light_context_t) -> bool {
    let mut target_path: [libc::c_char; 255] = [0; 255];
    _light_get_target_file(
        ctx,
        target_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
        b"save\0" as *const u8 as *const libc::c_char,
    );
    let mut saved_value: uint64_t = 0 as libc::c_int as uint64_t;
    if !light_file_read_uint64(target_path.as_mut_ptr(), &mut saved_value) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't read value from savefile\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                1050 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut mincap: uint64_t = _light_get_min_cap(ctx);
    if mincap > saved_value {
        saved_value = mincap;
    }
    if !((*(*ctx).run_params.device_target).set_value).expect("non-null function pointer")(
        (*ctx).run_params.device_target,
        saved_value,
    ) {
        if light_loglevel as libc::c_uint >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"%s:%d: Error: couldn't write saved value to device target\n\0" as *const u8
                    as *const libc::c_char,
                b"light.c\0" as *const u8 as *const libc::c_char,
                1062 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_create_device(
    mut enumerator: *mut light_device_enumerator_t,
    mut name: *const libc::c_char,
    mut device_data: *mut libc::c_void,
) -> *mut light_device_t {
    let mut new_device: *mut light_device_t =
        malloc(::std::mem::size_of::<light_device_t>() as libc::c_ulong) as *mut light_device_t;
    let ref mut fresh19 = (*new_device).enumerator;
    *fresh19 = enumerator;
    let ref mut fresh20 = (*new_device).targets;
    *fresh20 = 0 as *mut *mut light_device_target_t;
    (*new_device).num_targets = 0 as libc::c_int as uint64_t;
    let ref mut fresh21 = (*new_device).device_data;
    *fresh21 = device_data;
    snprintf(
        ((*new_device).name).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    _light_add_enumerator_device(enumerator, new_device);
    return new_device;
}
#[no_mangle]
pub unsafe extern "C" fn light_delete_device(mut device: *mut light_device_t) {
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < (*device).num_targets {
        light_delete_device_target(*((*device).targets).offset(i as isize));
        i = i.wrapping_add(1);
    }
    if !((*device).targets).is_null() {
        free((*device).targets as *mut libc::c_void);
    }
    if !((*device).device_data).is_null() {
        free((*device).device_data);
    }
    free(device as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn light_create_device_target(
    mut device: *mut light_device_t,
    mut name: *const libc::c_char,
    mut setfunc: LFUNCVALSET,
    mut getfunc: LFUNCVALGET,
    mut getmaxfunc: LFUNCMAXVALGET,
    mut cmdfunc: LFUNCCUSTOMCMD,
    mut target_data: *mut libc::c_void,
) -> *mut light_device_target_t {
    let mut new_target: *mut light_device_target_t =
        malloc(::std::mem::size_of::<light_device_target_t>() as libc::c_ulong)
            as *mut light_device_target_t;
    let ref mut fresh22 = (*new_target).device;
    *fresh22 = device;
    let ref mut fresh23 = (*new_target).set_value;
    *fresh23 = setfunc;
    let ref mut fresh24 = (*new_target).get_value;
    *fresh24 = getfunc;
    let ref mut fresh25 = (*new_target).get_max_value;
    *fresh25 = getmaxfunc;
    let ref mut fresh26 = (*new_target).custom_command;
    *fresh26 = cmdfunc;
    let ref mut fresh27 = (*new_target).device_target_data;
    *fresh27 = target_data;
    snprintf(
        ((*new_target).name).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    _light_add_device_target(device, new_target);
    return new_target;
}
#[no_mangle]
pub unsafe extern "C" fn light_delete_device_target(mut device_target: *mut light_device_target_t) {
    if !((*device_target).device_target_data).is_null() {
        free((*device_target).device_target_data);
    }
    free(device_target as *mut libc::c_void);
}
