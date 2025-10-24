use std::{ffi::CStr, mem, ptr};

// I know unsafe code sucks, but libc will be libc
pub fn get_user_by_uid(uid: u32) -> Option<String> {
    unsafe {
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512,
            n => n as usize,
        };

        let mut passwd: libc::passwd = mem::zeroed();
        let mut buf = Vec::with_capacity(amt);
        match libc::getpwuid_r(
            uid,
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity() as libc::size_t,
            &mut result,
        ) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_name as *const _;
                let username = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
                Some(username)
            }
            _ => None,
        }
    }
}

// This function is practically the same as the above
pub fn get_group_by_gid(gid: u32) -> Option<String> {
    unsafe {
        let mut result = ptr::null_mut();

        let amt = match libc::sysconf(libc::_SC_GETGR_R_SIZE_MAX) {
            n if n < 0 => 512,
            n => n as usize,
        };

        let mut group: libc::group = mem::zeroed();
        let mut buf = Vec::with_capacity(amt);

        match libc::getgrgid_r(
            gid,
            &mut group,
            buf.as_mut_ptr(),
            buf.capacity() as libc::size_t,
            &mut result,
        ) {
            0 if !result.is_null() => {
                let ptr = group.gr_name as *const _;
                let g_name = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
                Some(g_name)
            }
            _ => None,
        }
    }
}
