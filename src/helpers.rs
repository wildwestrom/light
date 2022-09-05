use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
}
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type size_t = libc::c_ulong;
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
#[no_mangle]
pub static mut light_loglevel: light_loglevel_t = 0 as light_loglevel_t;
#[no_mangle]
pub unsafe extern "C" fn light_file_read_uint64(
    mut filename: *const libc::c_char,
    mut val: *mut uint64_t,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut data: uint64_t = 0;
    fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        if light_loglevel as libc::c_uint
            >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Error: could not open '%s' for reading\n\0" as *const u8
                    as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                22 as libc::c_int,
                filename,
            );
        }
        if light_loglevel as libc::c_uint
            >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Error: Verify it exists with the right permissions\n\0"
                    as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                22 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if fscanf(
        fp,
        b"%lu\0" as *const u8 as *const libc::c_char,
        &mut data as *mut uint64_t,
    ) != 1 as libc::c_int
    {
        if light_loglevel as libc::c_uint
            >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Error: Couldn't parse an unsigned integer from '%s'\n\0"
                    as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                28 as libc::c_int,
                filename,
            );
        }
        fclose(fp);
        return 0 as libc::c_int != 0;
    }
    *val = data;
    fclose(fp);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_file_write_uint64(
    mut filename: *const libc::c_char,
    mut val: uint64_t,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        if light_loglevel as libc::c_uint
            >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Error: could not open '%s' for writing\n\0" as *const u8
                    as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                46 as libc::c_int,
                filename,
            );
        }
        if light_loglevel as libc::c_uint
            >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Error: Verify it exists with the right permissions\n\0"
                    as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                46 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if fprintf(fp, b"%lu\0" as *const u8 as *const libc::c_char, val) < 0 as libc::c_int
    {
        if light_loglevel as libc::c_uint
            >= LIGHT_ERROR_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Error: fprintf failed\n\0" as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int,
            );
        }
        fclose(fp);
        return 0 as libc::c_int != 0;
    }
    fclose(fp);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_file_exists(mut filename: *const libc::c_char) -> bool {
    return access(filename, 0 as libc::c_int) != -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn light_file_is_writable(
    mut filename: *const libc::c_char,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(filename, b"r+\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        if light_loglevel as libc::c_uint
            >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Warning: could not open '%s' for writing\n\0" as *const u8
                    as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int,
                filename,
            );
        }
        if light_loglevel as libc::c_uint
            >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Warning: Verify it exists with the right permissions\n\0"
                    as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    fclose(fp);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_file_is_readable(
    mut filename: *const libc::c_char,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        if light_loglevel as libc::c_uint
            >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Warning: could not open '%s' for reading\n\0" as *const u8
                    as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int,
                filename,
            );
        }
        if light_loglevel as libc::c_uint
            >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Warning: Verify it exists with the right permissions\n\0"
                    as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int,
            );
        }
        return 0 as libc::c_int != 0;
    }
    fclose(fp);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn light_log_clamp_min(mut min: uint64_t) -> uint64_t {
    if light_loglevel as libc::c_uint >= LIGHT_NOTE_LEVEL as libc::c_int as libc::c_uint
    {
        fprintf(
            stdout,
            b"%s:%d: Notice: too small value, adjusting to minimum %lu (raw)\n\0"
                as *const u8 as *const libc::c_char,
            b"helpers.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            min,
        );
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn light_log_clamp_max(mut max: uint64_t) -> uint64_t {
    if light_loglevel as libc::c_uint >= LIGHT_NOTE_LEVEL as libc::c_int as libc::c_uint
    {
        fprintf(
            stdout,
            b"%s:%d: Notice: too large value, adjusting to maximum %lu (raw)\n\0"
                as *const u8 as *const libc::c_char,
            b"helpers.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            max,
        );
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn light_percent_clamp(mut val: libc::c_double) -> libc::c_double {
    if val < 0.0f64 {
        if light_loglevel as libc::c_uint
            >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Warning: specified value %g%% is not valid, adjusting it to 0%%\n\0"
                    as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int,
                val,
            );
        }
        return 0.0f64;
    }
    if val > 100.0f64 {
        if light_loglevel as libc::c_uint
            >= LIGHT_WARN_LEVEL as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s:%d: Warning: specified value %g%% is not valid, adjusting it to 100%%\n\0"
                    as *const u8 as *const libc::c_char,
                b"helpers.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int,
                val,
            );
        }
        return 100.0f64;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn light_mkpath(
    mut dir: *mut libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if dir.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if stat(dir, &mut sb) == 0 {
        return 0 as libc::c_int;
    }
    let mut tempdir: *mut libc::c_char = strdup(dir);
    light_mkpath(dirname(tempdir), mode);
    free(tempdir as *mut libc::c_void);
    return mkdir(dir, mode);
}
